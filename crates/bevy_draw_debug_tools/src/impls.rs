use bevy::{prelude::Mesh, render::mesh::Indices, math::Vec3};
use bevy_rapier3d::prelude::{Collider, RayIntersection};


pub struct RayIntersectionContainer{
    pub toi: f32,
    pub point: Vec3,
    pub normal: Vec3,
}
pub trait RayIntersectionExt{
    fn get_ray_intersection(&self) -> RayIntersectionContainer;
}

impl RayIntersectionExt for RayIntersection{
    fn get_ray_intersection(&self) -> RayIntersectionContainer {
        RayIntersectionContainer{
            toi: self.toi,
            point: self.point,
            normal: self.normal,
        }
    }
}
pub trait MeshData{
    fn get_ddt_indices(&self) -> Option<&Vec<u32>>;

    fn get_ddt_vertices(&self) -> Option<&[[f32; 3]]>;
}


impl MeshData for Mesh{
    fn get_ddt_indices(&self) -> Option<&Vec<u32>> {
        if let Indices::U32(indices) = self.indices().unwrap(){
            return Some(indices);
        } else {
            None
        }
    }

    fn get_ddt_vertices(&self) -> Option<&[[f32; 3]]> {
        self.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3()
    }
}

impl MeshData for Collider{
    fn get_ddt_indices(&self) -> Option<&Vec<u32>> {
        let mesh = self.as_trimesh().unwrap();
        Some(&mesh.flat_indices().to_vec());
        todo!()
    }

    fn get_ddt_vertices(&self) -> Option<&[[f32; 3]]> {
        let typed_mesh = self.as_typed_shape();
        //let vertices = mesh.vertices().as.collect::<[f32;3]>();
        todo!()
    }
}
