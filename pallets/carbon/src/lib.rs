#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use scale_info::TypeInfo;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        sp_runtime::traits::Hash,
        traits::{ Randomness, Currency, tokens::ExistenceRequirement },
        transactional
    };
    use sp_io::hashing::blake2_128;

    #[cfg(feature = "std")]
    use frame_support::serde::{Deserialize, Serialize};

    // Struct to hold Credit information.
    type AccountOf<T> = <T as frame_system::Config>::AccountId;
    type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // Struct for holding Credit information.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    #[codec(mel_bound())]
    pub struct Credit<T: Config> {
        pub source: Source,
        pub serial_number: [u8; 64],
        pub for_sale: bool,
        pub retired: bool,
        pub owner: AccountOf<T>,
    }

    // Enum declaration for Source (3rd-party registries)
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub enum Source {
        VCS,
        GoldStandard,
        AmericanCarbonRegistry,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the Credits pallet.
        type Currency: Currency<Self::AccountId>;

        // MaxCreditsOwned constant
        #[pallet::constant]
        type MaxCreditsOwned: Get<u32>;
    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {

        /// Handles arithmetic overflow when incrementing the Credit counter.
        CountForCreditsOverflow,
        /// An account cannot own more Credits than `MaxCreditCount`.
        ExceedMaxCreditOwned,
        /// Buyer cannot be the owner.
        BuyerIsCreditOwner,
        /// Cannot transfer a credit to its owner.
        TransferToSelf,
        /// This credit already exists
        CreditExists,
        /// This credit doesn't exist
        CreditNotExist,
        /// Handles checking that the Credit is owned by the account transferring, buying or setting a price for it.
        NotCreditOwner,
        /// Ensures the Credit is for sale.
        CreditNotForSale,
        /// Ensures that the buying price is greater than the asking price.
        CreditBidPriceTooLow,
        /// Ensures that an account has enough funds to purchase a Credit.
        NotEnoughBalance,
    }

    // Events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {

        /// A new Credit was successfully created. \[sender, credit_id\]
        Created(T::AccountId, T::Hash),
        /// A Credit was successfully transferred. \[from, to, credit_id\]
        Transferred(T::AccountId, T::AccountId, T::Hash),
        /// A Credit was successfully bought. \[buyer, seller, credit_id, bid_price\]
        Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
    }

    #[pallet::storage]
    #[pallet::getter(fn count_for_credits)]
    /// Keeps track of the number of Credits in existence.
    pub(super) type CountForCredits<T: Config> = StorageValue<_, u64, ValueQuery>;

    // Storage items for credits
    #[pallet::storage]
    #[pallet::getter(fn credits)]
    pub(super) type Credits<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::Hash,
        Credit<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn credits_owned)]
    /// Keeps track of what accounts own which Credits.
    pub(super) type CreditsOwned<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BoundedVec<T::Hash, T::MaxCreditsOwned>,
        ValueQuery,
    >;

    // TODO Part IV: Our pallet's genesis configuration.

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        /// Create a new unique credit.
        ///
        /// The actual credit creation is done in the `mint()` function.
        #[pallet::weight(1)]
        pub fn create_credit(origin: OriginFor<T>,
                             source: Source,
                             serial_number: [u8; 64]
        ) -> DispatchResult {

            let sender = ensure_signed(origin)?;
            let credit_id = Self::mint(&sender, source, serial_number)?;
            // Logging to the console
            log::info!("A credit is created with ID: {:?}.", credit_id);

            // Deposit `Created` event
            Self::deposit_event(Event::Created(sender, credit_id));

            Ok(())
        }

        // TODO Part IV: set_price

        // TODO Part IV: transfer

        // TODO Part IV: buy_kitty

        // TODO Part IV: breed_kitty
    }

    //** Our helper functions.**//

    impl<T: Config> Pallet<T> {

        // TODO Part III: helper functions for dispatchable functions

        // Helper to mint a Credit.
        //
        // TODO Should prevent minting of credit with same source and serial number.
        pub fn mint(
            owner: &T::AccountId,
            source: Source,
            serial_number: [u8; 64]
        ) -> Result<T::Hash, Error<T>> {
            let credit = Credit::<T> {
                source,
                serial_number,
                for_sale: false,
                retired: false,
                owner: owner.clone(),
            };

            // TODO Replace this with hash of CreditIdentity struct component (once impl'd)
            let credit_id = T::Hashing::hash_of(&credit);

            // Performs this operation first as it may fail
            let new_cnt = Self::count_for_credits().checked_add(1)
                .ok_or(<Error<T>>::CountForCreditsOverflow)?;

            // Check if the credit does not already exist in our storage map
            ensure!(Self::credits(&credit_id) == None, <Error<T>>::CreditExists);

            // Performs this operation first because as it may fail
            <CreditsOwned<T>>::try_mutate(&owner, |credit_vec| {
                credit_vec.try_push(credit_id)
            }).map_err(|_| <Error<T>>::ExceedMaxCreditOwned)?;

            <Credits<T>>::insert(credit_id, credit);
            <CountForCredits<T>>::put(new_cnt);
            Ok(credit_id)
        }

        // Helper to check correct kitty owner
        pub fn is_credit_owner(credit_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
            match Self::credits(credit_id) {
                Some(credit) => Ok(credit.owner == *acct),
                None => Err(<Error<T>>::CreditNotExist)
            }
        }

        // TODO Part IV: transfer_kitty_to
    }
}