// Bierzemy Backend
#[cfg(not(target_os = "macos"))]
extern crate gfx_backend_vulkan as backend;
#[cfg(target_os = "macos")]
extern crate gfx_backend_metal as backend;

/* Używany jako "main", musi odebrać Reference do stanu gry który ma wyświetlić */
fn render() {

}