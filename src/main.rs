mod home;
mod navbar;
mod product_card;

use leptos::*;
use navbar::*;
use home::*;
use product_card::*;


fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
           <nav style="background-color:white;">
      <div class="logo">
        <h1>TWOOTER</h1>
      </div>
      <ul>
        <li>
          <a href="">Home</a>
        </li>
        <li>
          <a href="#">Services</a>
        </li>
        <li>
          <a href="#">Blog</a>
        </li>
        <li>
          <a href="#">Contact Us</a>
        </li>
      </ul>
      <div class="hamburger">
        <span class="line"></span>
        <span class="line"></span>
        <span class="line"></span>
      </div>
    </nav>
    <div class="menubar">
      <ul>
        <li>
          <a href="#">Home</a>
        </li>
        <li>
          <a href="#">Services</a>
        </li>
        <li>
          <a href="#">Blog</a>
        </li>
        <li>
          <a href="#">Contact Us</a>
        </li>
      </ul>
    </div>
            <ProductCard name=String::from("coucou")/>
        }
    })
}
