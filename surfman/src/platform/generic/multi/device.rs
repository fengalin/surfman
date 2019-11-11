// surfman/surfman/src/platform/generic/multi/device.rs
//
//! A device abstraction that allows the choice of backends dynamically.

use crate::{ContextID, Error, GLApi, SurfaceAccess, SurfaceInfo, SurfaceType};
use crate::connection::Connection as ConnectionInterface;
use crate::context::ContextAttributes;
use crate::device::Device as DeviceInterface;
use crate::gl::types::{GLenum, GLuint};
use super::adapter::Adapter;
use super::connection::Connection;
use super::context::{Context, ContextDescriptor};
use super::surface::{NativeWidget, Surface, SurfaceTexture};

use std::os::raw::c_void;

pub enum Device<Def, Alt> where Def: DeviceInterface, Alt: DeviceInterface {
    Default(Def),
    Alternate(Alt),
}

impl<Def, Alt> Device<Def, Alt> where Def: DeviceInterface,
                                      Alt: DeviceInterface,
                                      Def::Connection: ConnectionInterface,
                                      Alt::Connection: ConnectionInterface {
    pub fn adapter(&self) -> Adapter<Def, Alt> {
        match *self {
            Device::Default(ref device) => Adapter::Default(device.adapter()),
            Device::Alternate(ref device) => Adapter::Alternate(device.adapter()),
        }
    }

    pub fn connection(&self) -> Connection<Def, Alt> {
        match *self {
            Device::Default(ref device) => Connection::Default(device.connection()),
            Device::Alternate(ref device) => Connection::Alternate(device.connection()),
        }
    }

    pub fn gl_api(&self) -> GLApi {
        match *self {
            Device::Default(ref device) => device.gl_api(),
            Device::Alternate(ref device) => device.gl_api(),
        }
    }
}

impl<Def, Alt> DeviceInterface for Device<Def, Alt>
                               where Def: DeviceInterface,
                                     Alt: DeviceInterface,
                                     Def::Connection: ConnectionInterface<Device = Def>,
                                     Alt::Connection: ConnectionInterface<Device = Alt> {
    type Connection = Connection<Def, Alt>;
    type Context = Context<Def, Alt>;
    type ContextDescriptor = ContextDescriptor<Def, Alt>;
    type Surface = Surface<Def, Alt>;
    type SurfaceTexture = SurfaceTexture<Def, Alt>;

    // device.rs

    #[inline]
    fn connection(&self) -> Connection<Def, Alt> {
        Device::connection(self)
    }

    #[inline]
    fn adapter(&self) -> Adapter<Def, Alt> {
        Device::adapter(self)
    }

    #[inline]
    fn gl_api(&self) -> GLApi {
        Device::gl_api(self)
    }

    // context.rs

    #[inline]
    fn create_context_descriptor(&self, attributes: &ContextAttributes)
                                 -> Result<Self::ContextDescriptor, Error> {
        Device::create_context_descriptor(self, attributes)
    }

    #[inline]
    fn create_context(&mut self, descriptor: &ContextDescriptor<Def, Alt>)
                      -> Result<Context<Def, Alt>, Error> {
        Device::create_context(self, descriptor)
    }

    #[inline]
    fn destroy_context(&self, context: &mut Context<Def, Alt>) -> Result<(), Error> {
        Device::destroy_context(self, context)
    }

    #[inline]
    fn context_descriptor(&self, context: &Context<Def, Alt>) -> Self::ContextDescriptor {
        Device::context_descriptor(self, context)
    }

    #[inline]
    fn make_context_current(&self, context: &Context<Def, Alt>) -> Result<(), Error> {
        Device::make_context_current(self, context)
    }

    #[inline]
    fn make_no_context_current(&self) -> Result<(), Error> {
        Device::make_no_context_current(self)
    }

    #[inline]
    fn context_descriptor_attributes(&self, context_descriptor: &ContextDescriptor<Def, Alt>)
                                     -> ContextAttributes {
        Device::context_descriptor_attributes(self, context_descriptor)
    }

    #[inline]
    fn get_proc_address(&self, context: &Context<Def, Alt>, symbol_name: &str) -> *const c_void {
        Device::get_proc_address(self, context, symbol_name)
    }

    #[inline]
    fn bind_surface_to_context(&self, context: &mut Context<Def, Alt>, surface: Surface<Def, Alt>)
                               -> Result<(), Error> {
        Device::bind_surface_to_context(self, context, surface)
    }

    #[inline]
    fn unbind_surface_from_context(&self, context: &mut Context<Def, Alt>)
                                   -> Result<Option<Surface<Def, Alt>>, Error> {
        Device::unbind_surface_from_context(self, context)
    }

    #[inline]
    fn context_id(&self, context: &Context<Def, Alt>) -> ContextID {
        Device::context_id(self, context)
    }

    #[inline]
    fn context_surface_info(&self, context: &Context<Def, Alt>)
                            -> Result<Option<SurfaceInfo>, Error> {
        Device::context_surface_info(self, context)
    }

    // surface.rs

    #[inline]
    fn create_surface(&mut self,
                      context: &Context<Def, Alt>,
                      surface_access: SurfaceAccess,
                      surface_type: SurfaceType<NativeWidget<Def, Alt>>)
                      -> Result<Surface<Def, Alt>, Error> {
        Device::create_surface(self, context, surface_access, surface_type)
    }

    #[inline]
    fn create_surface_texture(&self, context: &mut Context<Def, Alt>, surface: Surface<Def, Alt>)
                              -> Result<SurfaceTexture<Def, Alt>, Error> {
        Device::create_surface_texture(self, context, surface)
    }

    #[inline]
    fn destroy_surface(&self, context: &mut Context<Def, Alt>, surface: Surface<Def, Alt>)
                       -> Result<(), Error> {
        Device::destroy_surface(self, context, surface)
    }

    #[inline]
    fn destroy_surface_texture(&self,
                               context: &mut Context<Def, Alt>,
                               surface_texture: SurfaceTexture<Def, Alt>)
                               -> Result<Surface<Def, Alt>, Error> {
        Device::destroy_surface_texture(self, context, surface_texture)
    }

    #[inline]
    fn surface_gl_texture_target(&self) -> GLenum {
        Device::surface_gl_texture_target(self)
    }

    #[inline]
    fn present_surface(&self, context: &Context<Def, Alt>, surface: &mut Surface<Def, Alt>)
                       -> Result<(), Error> {
        Device::present_surface(self, context, surface)
    }

    #[inline]
    fn surface_info(&self, surface: &Surface<Def, Alt>) -> SurfaceInfo {
        Device::surface_info(self, surface)
    }

    #[inline]
    fn surface_texture_object(&self, surface_texture: &SurfaceTexture<Def, Alt>) -> GLuint {
        Device::surface_texture_object(self, surface_texture)
    }
}
