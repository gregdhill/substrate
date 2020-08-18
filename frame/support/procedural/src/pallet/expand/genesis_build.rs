// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::pallet::Def;
use proc_macro2::Span;

/// * implement the trait `sp_runtime::BuildModuleGenesisStorage` using `__InherentHiddenInstance`
///   if needed
pub fn expand_module_interface(def: &mut Def) -> proc_macro2::TokenStream {
	let genesis_config = if let Some(genesis_config) = &def.genesis_config {
		genesis_config
	} else {
		return Default::default()
	};

	let scrate = &def.scrate();
	let type_impl_gen = &def.type_impl_generics();
	let type_use_gen = &def.type_use_generics();
	let trait_use_gen = if def.trait_.has_instance {
		quote::quote!(T, I)
	} else {
		let inherent_instance = syn::Ident::new(crate::INHERENT_INSTANCE_NAME, Span::call_site());
		quote::quote!(T, #inherent_instance)
	};
	let gen_cfg_ident = &genesis_config.genesis_config;
	let gen_cfg_use_gen = match (genesis_config.has_trait, genesis_config.has_instance) {
		(false, false) => quote::quote!(),
		(true, false) => quote::quote!(T),
		(false, true) => quote::quote!(I),
		(true, true) => quote::quote!(T, I),
	};

	quote::quote!(
		#[cfg(features = "std")]
		impl<#type_impl_gen> #scrate::sp_runtime::BuildModuleGenesisStorage<#trait_use_gen>
			for #gen_cfg_ident<#gen_cfg_use_gen>
		{
			fn build_module_genesis_storage(
				&self,
				storage: &mut #scrate::sp_runtime::Storage,
			) -> std::result::Result<(), std::string::String> {
				#scrate::BasicExternalities::execute_with_storage(storage, || {
					<Self as #scrate::traits::GenesisBuilder<#type_use_gen>>::build();
					Ok(())
				})
			}
		}
	)
}