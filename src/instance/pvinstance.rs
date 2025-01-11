use std::sync::Arc;

use crate::{core::{inheritance_cast_to, lua_macros::lua_invalid_argument, RwLockReadGuard, RwLockWriteGuard}, userdata::CFrame};
use mlua::prelude::*;

use super::{instance::IInstanceComponent, IInstance, ManagedInstance, WeakManagedInstance};

#[derive(Debug)]
pub struct PVInstanceComponent {
    origin: CFrame,
    pivot_offset: CFrame
}

impl PVInstanceComponent {
    
}

impl IInstanceComponent for PVInstanceComponent {
    fn lua_get(self: &mut RwLockReadGuard<'_, PVInstanceComponent>, _ptr: WeakManagedInstance, lua: &Lua, key: &String) -> Option<LuaResult<LuaValue>> {
        match key.as_str() {
            "GetPivot" => Some(Ok(LuaValue::Function(lua.create_function(|_, this: ManagedInstance| {
                let i = inheritance_cast_to!(&*this, dyn IPVInstance);
                i
                    .map(|x| x.get_pivot())
                    .map_err(|_|
                        lua_invalid_argument!("PVInstance::GetPivot",1,self cast Instance to PVInstance)
                    )
            }).unwrap()))),
            "PivotTo" => Some(Ok(LuaValue::Function(lua.create_function(|_, (this, cf): (ManagedInstance,CFrame)| {
                let i = inheritance_cast_to!(&*this, dyn IPVInstance);
                i
                    .map(|x| x.get_pivot())
                    .map_err(|_|
                        lua_invalid_argument!("PVInstance::PivotTo",1,self cast Instance to PVInstance)
                    )
            }).unwrap()))),
            _ => None
        }
    }

    fn lua_set(self: &mut RwLockWriteGuard<'_, PVInstanceComponent>, _ptr: WeakManagedInstance, _lua: &Lua, _key: &String, _value: &LuaValue) -> Option<LuaResult<()>> {
        None
    }

    fn clone(self: &RwLockReadGuard<'_, PVInstanceComponent>, _new_ptr: WeakManagedInstance) -> LuaResult<Self> {
        Ok(PVInstanceComponent {
            origin: self.origin,
            pivot_offset: self.pivot_offset
        })
    }

    fn new(_ptr: WeakManagedInstance, _class_name: &'static str) -> Self {
        PVInstanceComponent {
            origin: CFrame::new(),
            pivot_offset: CFrame::new()
        }
    }
}

pub trait IPVInstance: IInstance {
    fn get_pv_instance_component(&self) -> RwLockReadGuard<'_, PVInstanceComponent>;
    fn get_pv_instance_component_mut(&self) -> RwLockWriteGuard<'_, PVInstanceComponent>;
}

impl dyn IPVInstance {
    pub fn get_pivot(&self) -> CFrame {
        todo!()
    }
    pub fn pivot_to(&mut self, pivot: CFrame)  {
        todo!()
    }
}

