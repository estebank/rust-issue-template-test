// Modules generated by transparent proc macros still acts as barriers for names (issue #50504).

// aux-build:generate-mod.rs

extern crate generate_mod;

struct FromOutside;

generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
                        //~| ERROR cannot find type `Outer` in this scope

#[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
                            //~| ERROR cannot find type `OuterAttr` in this scope
struct S;

#[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
                                     //~| ERROR cannot find type `OuterDerive` in this scope
struct Z;

fn inner_block() {
    #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
                                        //~| ERROR cannot find type `OuterDerive` in this scope
    struct InnerZ;
}

#[derive(generate_mod::CheckDeriveLint)] //~  ERROR cannot find type `OuterDeriveLint` in this scope
                                         //~| ERROR cannot find type `FromOutside` in this scope
struct W;

fn main() {}
