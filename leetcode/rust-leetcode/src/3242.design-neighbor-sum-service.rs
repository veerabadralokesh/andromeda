struct NeighborSum {
    adj: Vec<i32>,
    dia: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {

    fn new(grid: Vec<Vec<i32>>) -> Self {
        let n = grid.len();
        let mut adj = vec![0; n*n];
        let mut dia = vec![0; n*n];
        let mut g = vec![vec![0; n+2]; n+2];
        for i in 0..n {
            for j in 0..n {
                g[i+1][j+1] = grid[i][j];
            }
        }
        let (mut a, mut d, mut idx) = (0, 0, 0);
        for i in 1..=n {
            for j in 1..=n {
                idx = g[i][j] as usize;
                adj[idx] = g[i-1][j] + g[i+1][j] + g[i][j-1] + g[i][j+1];
                dia[idx] = g[i-1][j-1] + g[i-1][j+1] + g[i+1][j+1] + g[i+1][j-1];
            }
        }
        Self {adj, dia}
    }
    
    fn adjacent_sum(&self, value: i32) -> i32 {
        self.adj[value as usize]
    }
    
    fn diagonal_sum(&self, value: i32) -> i32 {
        self.dia[value as usize]
    }
}

/**
 * Your NeighborSum object will be instantiated and called as such:
 * let obj = NeighborSum::new(grid);
 * let ret_1: i32 = obj.adjacent_sum(value);
 * let ret_2: i32 = obj.diagonal_sum(value);
 */

/* */

// LEARN


pub use __cargo_equip::prelude::*;

use kyopro_grid::P;
use kyopro_utils::*;

struct neighborSum {
    g: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl neighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        Self { g: grid }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        let N = self.g.len();
        for i in 0..N {
            for j in 0..N {
                if self.g[i][j] == value {
                    let p = P(i, j);
                    let mut tot = 0;
                    for nv in p.adj4() {
                        if nv.0 < N && nv.1 < N {
                            tot += self.g[nv];
                        }
                    }
                    return tot;
                }
            }
        }
        return 0;
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        pub const DIAG: [P; 4] = [P(1, 1), P(1, !0), P(!0, 1), P(!0, !0)];

        let N = self.g.len();
        for i in 0..N {
            for j in 0..N {
                if self.g[i][j] == value {
                    let p = P(i, j);
                    let mut tot = 0;
                    for d in DIAG {
                        let np = p.add(&d);
                        if np.0 < N && np.1 < N {
                            tot += self.g[np];
                        }
                    }
                    return tot;
                }
            }
        }
        return 0;
    }
}

fn _main() {}

// The following code was expanded by `cargo-equip`.

///  # Bundled libraries
/// 
///  - `kyopro-grid 0.1.0 (path+██████████████████████████████████████████████████)`   published in **missing** licensed under `CC0-1.0` as `crate::__cargo_equip::crates::kyopro_grid`
///  - `kyopro-utils 0.1.0 (path+███████████████████████████████████████████████████)` published in **missing** licensed under `CC0-1.0` as `crate::__cargo_equip::crates::kyopro_utils`
#[cfg_attr(any(), rustfmt::skip)]
#[allow(unused)]
mod __cargo_equip {
    pub(crate) mod crates {
        pub mod kyopro_grid {use std::{fmt,ops::{self,Index,IndexMut},};pub const D4:[P;4]=[P(1,0),P(0,1),P(!0,0),P(0,!0)];#[derive(Clone,Copy,PartialEq,Eq,Debug)]pub enum Direction{D,R,U,L,}impl Direction{pub fn rev(&self)->Self{match self{Direction::D=>Direction::U,Direction::U=>Direction::D,Direction::R=>Direction::L,Direction::L=>Direction::R,}}}impl From<usize>for Direction{fn from(d:usize)->Self{match d{0=>Direction::D,1=>Direction::R,2=>Direction::U,3=>Direction::L,_=>unreachable!(),}}}impl fmt::Display for Direction{fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{let c=match self{Direction::D=>'D',Direction::R=>'R',Direction::U=>'U',Direction::L=>'L',};write!(f,"{}",c)}}#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash,PartialOrd,Ord)]pub struct P(pub usize,pub usize);impl P{pub fn adj4(self)->impl Iterator<Item=P>{D4.iter().map(move|&d|self.add(&P(d.0,d.1)))}pub fn add(self,rhs:&P)->P{P(self.0.wrapping_add(rhs.0),self.1.wrapping_add(rhs.1))}pub fn dist(&self,rhp:&P)->usize{let a=(self.0 as i32-rhp.0 as i32).abs()as usize;let b=(self.1 as i32-rhp.1 as i32).abs()as usize;a+b}}impl ops::Add<Direction>for P{type Output=Self;fn add(self,rhs:Direction)->Self{match rhs{Direction::D=>self.add(&D4[0]),Direction::R=>self.add(&D4[1]),Direction::U=>self.add(&D4[2]),Direction::L=>self.add(&D4[3]),}}}impl<T>Index<P>for Vec<Vec<T>>{type Output=T;fn index(&self,p:P)->&T{&self[p.0][p.1]}}impl<T>IndexMut<P>for Vec<Vec<T>>{fn index_mut(&mut self,p:P)->&mut T{&mut self[p.0][p.1]}}#[derive(Debug,Clone)]pub struct Grid<T>{h:usize,w:usize,pub g:Vec<Vec<T>>,}impl<T>Grid<T>where T:Clone,{pub fn from_vec(g:Vec<Vec<T>>)->Self{Grid{h:g.len(),w:g[0].len(),g,}}#[allow(non_snake_case)]pub fn new(h:usize,w:usize,I:T)->Self{let g=vec![vec![I;w];h];Grid{h,w,g}}}impl<T>Grid<T>{pub fn adj4<'a>(&'a self,p:P)->impl Iterator<Item=P>+'a{D4.iter().map(move|&d|p.add(&P(d.0,d.1))).filter(|&p|p.0<self.h&&p.1<self.w)}pub fn positions_from<'a>(&'a self,b:P,offsets:&'a[P])->impl Iterator<Item=P>+'a{let x=offsets.iter().map(move|&d|P(b.0.wrapping_add(d.0),b.1.wrapping_add(d.1)));x.filter(|&p|p.0<self.h&&p.1<self.w)}pub fn position_from(&self,b:P,offset:P)->Option<P>{let x=b.add(&offset);if self.is_valid_position(&x){Some(x)}else{None}}pub fn is_valid_position(&self,p:&P)->bool{p.0<self.h&&p.1<self.w}}impl<T>Index<usize>for Grid<T>{type Output=[T];#[inline]fn index(&self,idx:usize)->&[T]{&self.g[idx]}}impl<T>IndexMut<usize>for Grid<T>{#[inline]fn index_mut(&mut self,idx:usize)->&mut[T]{&mut self.g[idx]}}impl<T>Index<P>for Grid<T>{type Output=T;#[inline]fn index(&self,idx:P)->&T{&self.g[idx.0][idx.1]}}impl<T>IndexMut<P>for Grid<T>{#[inline]fn index_mut(&mut self,p:P)->&mut T{&mut self.g[p.0][p.1]}}pub fn rot_clock<T>(vec:&Vec<Vec<T>>)->Vec<Vec<T>>where T:Copy+Default,{let h=vec.len();let w=vec[0].len();let mut res=vec![vec![T::default();h];w];for i in 0..w{for j in 0..h{res[i][j]=vec[h-1-j][i];}}res}pub fn rot_rev_clock<T>(vec:&Vec<Vec<T>>)->Vec<Vec<T>>where T:Copy+Default,{let h=vec.len();let w=vec[0].len();let mut res=vec![vec![T::default();h];w];for i in 0..w{for j in 0..h{res[i][j]=vec[j][w-i-1];}}res}}
        pub mod kyopro_utils {pub use crate::__cargo_equip::macros::kyopro_utils::*;use std::ops::{Add,Rem};#[macro_export]macro_rules!__cargo_equip_macro_def_kyopro_utils_mat{($($e:expr),*)=>{Vec::from(vec![$($e),*])};($($e:expr,)*)=>{Vec::from(vec![$($e),*])};($e:expr;$d:expr)=>{Vec::from(vec![$e;$d])};($e:expr;$d:expr$(;$ds:expr)+)=>{Vec::from(vec![mat![$e$(;$ds)*];$d])};}macro_rules!mat{($($tt:tt)*)=>(crate::__cargo_equip_macro_def_kyopro_utils_mat!{$($tt)*})}#[macro_export]macro_rules!__cargo_equip_macro_def_kyopro_utils_ec{($($num:expr),*)=>{let mut tmp=vec![];$(tmp.push(format!("{}",$num));)*print!("{}
",tmp.join(" "));};}macro_rules!ec{($($tt:tt)*)=>(crate::__cargo_equip_macro_def_kyopro_utils_ec!{$($tt)*})}#[macro_export]macro_rules!__cargo_equip_macro_def_kyopro_utils_YesNo{($num:expr)=>{if($num)as i64==0{println!("No");}else{println!("Yes");}};}macro_rules!YesNo{($($tt:tt)*)=>(crate::__cargo_equip_macro_def_kyopro_utils_YesNo!{$($tt)*})}#[macro_export]macro_rules!__cargo_equip_macro_def_kyopro_utils_Yes{()=>{println!("Yes");};}macro_rules!Yes{($($tt:tt)*)=>(crate::__cargo_equip_macro_def_kyopro_utils_Yes!{$($tt)*})}#[macro_export]macro_rules!__cargo_equip_macro_def_kyopro_utils_No{()=>{println!("No");};}macro_rules!No{($($tt:tt)*)=>(crate::__cargo_equip_macro_def_kyopro_utils_No!{$($tt)*})}pub trait SetMinMax{fn setmin(&mut self,v:Self)->bool;fn setmax(&mut self,v:Self)->bool;}impl<T>SetMinMax for T where T:PartialOrd,{fn setmin(&mut self,v:T)->bool{*self>v&&{*self=v;true}}fn setmax(&mut self,v:T)->bool{*self<v&&{*self=v;true}}}#[macro_export]macro_rules!__cargo_equip_macro_def_kyopro_utils_chmax{($lhs:expr,$rhs:expr)=>{if$lhs<$rhs{let tmp=$rhs;$lhs=tmp;true}else{false}};}macro_rules!chmax{($($tt:tt)*)=>(crate::__cargo_equip_macro_def_kyopro_utils_chmax!{$($tt)*})}#[macro_export]macro_rules!__cargo_equip_macro_def_kyopro_utils_chmin{($lhs:expr,$rhs:expr)=>{if$lhs>$rhs{let tmp=$rhs;$lhs=tmp;true}else{false}};}macro_rules!chmin{($($tt:tt)*)=>(crate::__cargo_equip_macro_def_kyopro_utils_chmin!{$($tt)*})}pub fn print_vec<T>(v:&[T])where T:std::fmt::Display,{for i in 0..v.len(){print!("{}{}",v[i],if i+1==v.len(){""}else{" "});}println!();}pub fn pmod<T:Copy+Add<Output=T>+Rem<Output=T>>(x:T,m:T)->T{((x%m)+m)%m}pub fn lower_bound<T>(a:&[T],x:&T)->usize where T:Ord,{if a.len()==0||a[0]>=*x{return 0;}let mut l=0;let mut r=a.len();while l+1<r{let m=(l+r)/2;if a[m]<*x{l=m;}else{r=m;}}r}pub fn upper_bound<T>(a:&[T],x:&T)->usize where T:Ord,{if a.len()==0||a[0]>*x{return 0;}let mut l=0;let mut r=a.len();while l+1<r{let m=(l+r)/2;if a[m]<=*x{l=m;}else{r=m;}}r}#[allow(unused_macros)]#[macro_export]macro_rules!__cargo_equip_macro_def_kyopro_utils_db{($($a:expr),*$(,)*)=>{#[cfg(debug_assertions)]eprintln!(concat!($("| ",stringify!($a),"={:?} "),*),$(&$a),*);};}macro_rules!db{($($tt:tt)*)=>(crate::__cargo_equip_macro_def_kyopro_utils_db!{$($tt)*})}#[allow(unused_macros)]#[macro_export]macro_rules!__cargo_equip_macro_def_kyopro_utils_db2d{($vec:expr)=>{#[cfg(debug_assertions)]{eprintln!("> {}=",stringify!($vec));for a in$vec.iter(){eprintln!("> {:?}",a);}}};}macro_rules!db2d{($($tt:tt)*)=>(crate::__cargo_equip_macro_def_kyopro_utils_db2d!{$($tt)*})}#[derive(PartialEq,PartialOrd,Clone,Copy)]pub struct OrdF<T>(pub T);impl<T:PartialEq>Eq for OrdF<T>{}impl<T:PartialOrd>Ord for OrdF<T>{fn cmp(&self,other:&OrdF<T>)->std::cmp::Ordering{self.0.partial_cmp(&other.0).unwrap()}}}
    }

    pub(crate) mod macros {
        pub mod kyopro_grid {}
        pub mod kyopro_utils {pub use crate::{__cargo_equip_macro_def_kyopro_utils_No as No,__cargo_equip_macro_def_kyopro_utils_Yes as Yes,__cargo_equip_macro_def_kyopro_utils_YesNo as YesNo,__cargo_equip_macro_def_kyopro_utils_chmax as chmax,__cargo_equip_macro_def_kyopro_utils_chmin as chmin,__cargo_equip_macro_def_kyopro_utils_db as db,__cargo_equip_macro_def_kyopro_utils_db2d as db2d,__cargo_equip_macro_def_kyopro_utils_ec as ec,__cargo_equip_macro_def_kyopro_utils_mat as mat};}
    }

    pub(crate) mod prelude {pub use crate::__cargo_equip::crates::*;}

    mod preludes {
        pub mod kyopro_grid {}
        pub mod kyopro_utils {}
    }
}


/**
 * Your neighborSum object will be instantiated and called as such:
 * let obj = neighborSum::new(grid);
 * let ret_1: i32 = obj.adjacent_sum(value);
 * let ret_2: i32 = obj.diagonal_sum(value);
 */

