const js = import("./pkg/rust_rect.js");
try {
  js.then(js => {
    js.greet();
 });
} catch(e) {
  console.log(e);
}