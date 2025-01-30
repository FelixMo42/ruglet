use wgpu::*;

pub struct Texture {
    pub view: TextureView,
    pub sampler: Sampler,
}

pub fn create_texture(
    device: &Device,
    queue: &Queue,
    bytes: &[u8],
    dimensions: (u32, u32),
) -> Texture {
    let texture_size = Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        // All textures are stored as 3D, we represent our 2D texture
        // by setting depth to 1.
        depth_or_array_layers: 1,
    };

    let diffuse_texture = device.create_texture(&TextureDescriptor {
        label: Some("diffuse_texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,

        // Most images are stored using sRGB, so we need to reflect that here.
        format: TextureFormat::Rgba8UnormSrgb,

        // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
        // COPY_DST means that we want to copy data to this texture
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,

        // This is the same as with the SurfaceConfig. It
        // specifies what texture formats can be used to
        // create TextureViews for this texture. The base
        // texture format (Rgba8UnormSrgb in this case) is
        // always supported. Note that using a different
        // texture format is not supported on the WebGL2
        // backend.
        view_formats: &[],
    });

    queue.write_texture(
        // Tells wgpu where to copy the pixel data
        TexelCopyTextureInfo {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        // The actual pixel data
        &bytes,
        // The layout of the texture
        TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        texture_size,
    );

    // We don't need to configure the texture view much, so let's let wgpu define it.
    let view = diffuse_texture.create_view(&TextureViewDescriptor::default());
    let sampler = device.create_sampler(&SamplerDescriptor::default());

    return Texture { view, sampler };
}
