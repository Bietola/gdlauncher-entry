#![feature(iter_next_chunk)]

pub mod no_idx;
pub mod no_dups;
pub mod cyclic;
pub mod w_counter;
pub mod counter_n_cyclic;

#[macro_export]
macro_rules! mod_solve {
    ($mdl:ident) => {
        $mdl::solve()
    };
}
