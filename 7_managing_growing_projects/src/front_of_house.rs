// A mod referenced outside of its root needs to be made public.
pub mod hosting {
  // Functions inside a mod are not visible to other functions outside of the mod, so it too needs to be made public if referenced outside of the mod.
  pub fn add_to_waitlist() {
    // A child can see the functions etc. defined by it's ancestors, but the contents of the child are not visible to the ancestors, if not specifically made public.
  }
}
