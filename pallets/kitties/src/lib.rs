#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Randomness};
    use frame_system::pallet_prelude::*;
    use codec::{Encode, Decode};
    use sp_io::hashing::blake2_128;

    #[derive(Encode, Decode)]
    pub struct Kitty(pub [u8; 16]);
    //a1,0555,定义一个16字节的u8类型来存储小猫,这样可以通过256位哈希函数获取.

    type KittyIndex = u32;
    //a2,定义小猫id.

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        //b12
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
        //b12,1809e 报错看到有一个random没有实现,因为在需要这个random时并没有把它绑定到外面的一个可以阐释这个random数据的一个实体上去.
        //解决方法是在config定义时声明一个random的实现.这个random满足这个trait. 而且它是以它本身的hash和blocknumber作为类型参数.(暂时没搞懂)
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
    //b11.1
        KittyCreate(T::AccountId, KittyIndex),
    //b11.1,1705e 把之前用到的数据,如error(b11.2),event加到对应的定义里去. 但视频尝试此时编译会报错associated type `Randomness' not found
    }

    #[pallet::storage]
    #[pallet::getter(fn kitties_count)]
    pub type KittiesCount<T> = StorageValue<_, u32>;
    //a3,要记录每个kitty的index,就要记录kitty的总数.通过此方法可以获得当前的总数值.

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty>, ValueQuery>;
    //a4,存储kitties数据,以index为key.数据为value存储到map中.Blake2_128Concat是哈希函数的名字,映射时使用它为方法.

    #[pallet::storage]
    #[pallet::getter(fn owner)]
    pub type Owner<T: Config> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;
    //a5,0853e 存储owner,与上条相同,用id作为key.但存储值为AccountId

    #[pallet::error]
    pub enum Error<T> {
    //b11.2
        KittiesCountOverflow,
    //b11.2
    }

    #[pallet::call]
    impl<T:Config> Pallet<T> {
    //b6
        #[pallet::weight(0)]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
    //b6,extrinisc返回值是DispatchResult.这里如果正确则返回调用方法的accountid给who.
    //b7
            let kitty_id = match Self::kitties_count() {
            Some(id) => {
                ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
                id
            },
            None => {
                1
            }
        };
    //b7,1016-1146 创建时需要取得下一个kitty的id,所以需要把a3的kittiescount值取出来.这里是实现逻辑.如果达到max_value,则抛出KittiesCountOverflow Error.
    //b8.1
        let dna = Self::random_value(&who);
    //b8.1
    //b9
        Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
        Owner::<T>::insert(kitty_id, Some(who.clone()));
        KittiesCount::<T>::put(kitty_id + 1);
    //b9,1450e 数据已经准备好了,现在要把数据放到区块链上去. 数据包括kitty的dna, owner,以及kitty id.下一个kittyid应该是现在的基础设加1.
    //b10
        Self::deposit_event(Event::KittyCreate(who, kitty_id));
        Ok(())
    //b10,1533e 最后需要对外抛出一个event,让前端js或dapp程序知道有个新kitty创建出来了.并且kitty的owner和id也会放到event里去.最后函数返回ok.

    }
    }

    impl<T: Config> Pallet<T> {
    //b8.2
        fn random_value(sender: &T::AccountId) -> [u8; 16] {
            let payload = (
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }
    //b8.2,1146-1345 有了id后就要取得kitty的data(dna),通过随机的方式获取.b8.1定义了获取随机值的方法.b8.2是方法的实现过程.前端最终用这些数据做展示.具体看视频.
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
}

