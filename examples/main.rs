use render_mate::Camera;
use render_mate::Color;
use render_mate::Extent;
use render_mate::Framebuffer;
use render_mate::Mat4;
use render_mate::Mesh;
use render_mate::Scene;
use render_mate::Sphere;
use render_mate::Texture;
use render_mate::Vec3;
use render_mate::DiffuseMaterial;
use render_mate::LightMaterial;
use render_mate::MirrorMaterial;
use render_mate::Node;

fn main() {
    let mut scene = Scene::new();

    let grey_texture = Texture::from("data/textures/grey_texture.png");
    let white_texture = Texture::from("data/textures/white_texture.bmp");
    let red_texture = Texture::from("data/textures/red_texture.png");
    let green_texture = Texture::from("data/textures/green_texture.png");
    let blue_texture = Texture::from("data/textures/blue_texture.png");
    let yellow_texture = Texture::from("data/textures/yellow_texture.png");
    let null_texture = Texture::from("data/textures/null_texture.bmp");

    let white_diffuse_material = DiffuseMaterial {
        texture: &white_texture,
    };

    let null_diffuse_material = DiffuseMaterial {
        texture: &null_texture,
    };

    let mirror_material = MirrorMaterial {

    };

    let white_light_material = LightMaterial {
        color: Color::new(
            1.0,
            1.0,
            1.0,
            1.0,
        ),
        intensity: 20.0,
    };

    let sky_box_geometry = Mesh::new_cube(
        &white_diffuse_material,
        Mat4::new_translation(Vec3::new(0.0, 0.0, 0.0))*Mat4::new_scale(Vec3::new(3.0, 3.0, 3.0)),
    );
    let sky_box_node = Node {
        geometry: &sky_box_geometry,
        material: &white_diffuse_material,
    };
    scene.add_node(sky_box_node);

    //let light0 = Mesh::new_cube(
    //    &white_light_material,
    //    Mat4::new_translation(Vec3::new(0.0, -0.75, 1.0))*Mat4::new_scale(Vec3::new(0.25, 0.25, 0.25)),
    //);
    //scene.add_node(&light0);
    let light0 = Sphere {
        position: Vec3::new(0.0, -0.75, 1.0),
        radius: 0.125,
    };
    let light0_node = Node {
        geometry: &light0,
        material: &white_light_material,
    };
    scene.add_node(light0_node);


    let box0 = Mesh::new_cube(
        &null_diffuse_material,
        Mat4::new_translation(Vec3::new(-0.75, -1.0, 1.0))*Mat4::new_scale(Vec3::new(0.5, 1.0, 0.5))*Mat4::new_rotation_y(3.14/8.0),
    );
    let box0_node = Node {
        geometry: &box0,
        material: &null_diffuse_material,
    };
    scene.add_node(box0_node);

    let box1 = Mesh::new_cube(
        &null_diffuse_material,
        Mat4::new_translation(Vec3::new(0.75, -1.0, 1.0))*Mat4::new_scale(Vec3::new(0.5, 1.0, 0.5))*Mat4::new_rotation_y(-3.14/8.0),
    );
    let box1_node = Node {
        geometry: &box1,
        material: &null_diffuse_material,
    };
    scene.add_node(box1_node);


    let mirror_sphere = Sphere::new(
        Vec3::new(0.0, -1.25, 1.0),
        0.25,
    );
    let mirror_sphere_node = Node {
        geometry: &mirror_sphere,
        material: &mirror_material,
    };
    scene.add_node(mirror_sphere_node);


    /*let mirror0 = Mesh::new_cube(
        &mirror_material,
        Mat4::new_translation(Vec3::new(0.0, 0.0, 1.5))*Mat4::new_scale(Vec3::new(3.0, 3.0, 0.1)),
    );
    scene.add_node(&mirror0);

    let mirror1 = Mesh::new_cube(
        &mirror_material,
        Mat4::new_translation(Vec3::new(0.0, 0.0, -1.5))*Mat4::new_scale(Vec3::new(3.0, 3.0, 0.1)),
    );
    scene.add_node(&mirror1);*/

    /*let mirror_sphere = Sphere::new(
        Vec3::new(0.0, -1.25, 1.0),
        0.25,
        Color::new(1.0, 1.0, 1.0, 1.0),
        0.0,
        0.0,
    );
    scene.add_node(&mirror_sphere);*/

    let framebuffer_extent = Extent::new(512, 512);

    let camera = Camera::new(framebuffer_extent);

    let mut fb0 = Framebuffer::new(framebuffer_extent);
    let mut fb1 = Framebuffer::new(framebuffer_extent);
    let mut fb2 = Framebuffer::new(framebuffer_extent);
    let mut fb3 = Framebuffer::new(framebuffer_extent);
    let mut fb4 = Framebuffer::new(framebuffer_extent);
    let mut fb5 = Framebuffer::new(framebuffer_extent);
    let mut fb6 = Framebuffer::new(framebuffer_extent);
    let mut fb7 = Framebuffer::new(framebuffer_extent);

    std::thread::scope(|scope| {
        scope.spawn(|| {
            scene.render(&camera, &mut fb0);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb1);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb2);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb3);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb4);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb5);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb6);
        });
        scope.spawn(|| {
            scene.render(&camera, &mut fb7);
        });
    });

    fb0.merge(&fb1);
    fb2.merge(&fb3);
    fb4.merge(&fb5);
    fb6.merge(&fb7);

    fb0.merge(&fb2);
    fb4.merge(&fb6);

    fb0.merge(&fb4);

    fb0.save("res.png");
}