# Texture bag
This library provides a container to store textures without manual file loading and conversion boilerplate. 
It also provides single access point to textures, even if they weren't loaded on container init.
Library supports lazy loading to load textures from a file on-demand and eager loading to load all textures on container init.

## Usage
Add to your `cargo.toml`
```toml
[dependencies]
texture_bag = "0.0.1"
```

### Preparations
First of all, you need to create config file with list of all textures used in your project:
```json
{
  "textures": {
    "texture_id": "path_to_texture"    
  }
}
```
Default filename is `texture_config.json`, but you can change it passing actual name and path as one of the parameters on TextureBag init.

### Examples of usage
```rust
fn main() {
    // Some preparation code for proper init
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let window_builder = glium::glutin::window::WindowBuilder::new().with_title("Foo");
    let context = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window_builder, context, &event_loop).unwrap();

    // This call will load all textures from config into memory 
    let mut texture_bag = TextureBag::init_eager(&display, None);
    
    // Lazy init will load only config data for further operations, no texture loading will be made
    let mut texture_bag = TextureBag::init_lazy(&display, None);

    // Method returns reference to a texture. 
    // If texture was not loaded before, method will check config, load texture by path and store it into the bag.
    let texture = texture_bag.get_texture(String::from("texture_id"), display);
    
    // This method will remove texture from the bag. Texture can be loaded again by calling get_texture.
    texture_bag.forget(String::from("texture_id"));
}
```

## Further TODOs and issues
Please note that this version is basically alpha version of the project. API might change before it will be stabilized.
Further possible improvements:
- For now texture_id is basically a String. I probably need to make it more generic and support any type that implemented Display interface
- "All or nothing" loading is good for small number of files. I need to implement chunk loading for big projects. Loading textures by groups seems good enough.