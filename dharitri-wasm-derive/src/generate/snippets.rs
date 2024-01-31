pub fn contract_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct ContractObj<A>
        where
            A: dharitri_wasm::api::VMApi,
        {
            _phantom: core::marker::PhantomData<A>,
        }
    }
}

pub fn impl_contract_base() -> proc_macro2::TokenStream {
    quote! {
        impl<A> dharitri_wasm::contract_base::ContractBase for ContractObj<A>
        where
            A: dharitri_wasm::api::VMApi,
        {
            type Api = A;
        }
    }
}

pub fn new_contract_object_fn() -> proc_macro2::TokenStream {
    quote! {
        pub fn contract_obj<A>() -> ContractObj<A>
        where
            A: dharitri_wasm::api::VMApi,
        {
            ContractObj {
                _phantom: core::marker::PhantomData,
            }
        }

        pub struct ContractBuilder;

        impl dharitri_wasm::contract_base::CallableContractBuilder for self::ContractBuilder {
            fn new_contract_obj<A: dharitri_wasm::api::VMApi>(
                &self,
            ) -> dharitri_wasm::Box<dyn dharitri_wasm::contract_base::CallableContract> {
                dharitri_wasm::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
    }
}

// TODO: explore auto-implementations of supertraits
#[allow(dead_code)]
pub fn impl_auto_impl() -> proc_macro2::TokenStream {
    quote! {
        impl<A> AutoImpl for ContractObj<A> where
            A: dharitri_wasm::contract_base::ContractBase
                + dharitri_wasm::api::ErrorApi
                + dharitri_wasm::api::EndpointArgumentApi
                + dharitri_wasm::api::EndpointFinishApi
                + dharitri_wasm::api::ManagedTypeApi
        {
        }
    }
}

pub fn impl_callable_contract() -> proc_macro2::TokenStream {
    quote! {
        impl<A> dharitri_wasm::contract_base::CallableContract for ContractObj<A>
        where
            A: dharitri_wasm::api::VMApi,
        {
            fn call(&self, fn_name: &[u8]) -> bool {
                EndpointWrappers::call(self, fn_name)
            }

            fn clone_obj(&self) -> dharitri_wasm::Box<dyn dharitri_wasm::contract_base::CallableContract> {
                dharitri_wasm::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
    }
}

pub fn proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct Proxy<A>
        where
            A: dharitri_wasm::api::VMApi + 'static,
        {
            pub address: dharitri_wasm::types::ManagedAddress<A>,
        }

        impl<A> dharitri_wasm::contract_base::ProxyObjBase for Proxy<A>
        where
            A: dharitri_wasm::api::VMApi + 'static,
        {
            type Api = A;

            fn new_proxy_obj() -> Self {
                let zero_address = ManagedAddress::zero();
                Proxy {
                    address: zero_address,
                }
            }

            fn contract(mut self, address: ManagedAddress<Self::Api>) -> Self {
                self.address = address;
                self
            }

            #[inline]
            fn into_fields(self) -> ManagedAddress<Self::Api> {
                self.address
            }
        }
    }
}

pub fn callback_proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct CallbackProxyObj<A>
        where
            A: dharitri_wasm::api::VMApi + 'static,
        {
            _phantom: core::marker::PhantomData<A>,
        }

        impl<A> dharitri_wasm::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
        where
            A: dharitri_wasm::api::VMApi + 'static,
        {
            type Api = A;

            fn new_cb_proxy_obj() -> Self {
                CallbackProxyObj {
                    _phantom: core::marker::PhantomData,
                }
            }
        }
    }
}

pub fn call_method_api_static_init() -> proc_macro2::TokenStream {
    quote! {
        <Self::Api as dharitri_wasm::api::VMApi>::init_static();
    }
}
