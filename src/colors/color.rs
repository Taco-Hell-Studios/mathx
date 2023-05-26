
use crate::Math;

/// A structure for a color where each channel is a floating point value between 0.0 and 1.0
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Color {
	/// The red channel of the color
	r: f32,
	/// The green channel of the color
	g: f32,
	/// The blue channel of the color
	b: f32,
	/// The alpha channel of the color
	a: f32,
}

/// Constructors
impl Color {
	/// Creates a new color using rgb with floating point numbers
	/// - **r**: The red channel to set
	/// - **g**: The green channel to set
	/// - **b**: The blue channel to set
	/// 
	/// **Returns**: Returns a new color using rgb with floating point numbers
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let rgb = Color::new(0.5, 0.75, 0.4980392157);
	/// assert_eq!(0.5, rgb.red());
	/// assert_eq!(127, rgb.red_as_byte());
	/// assert_eq!(0.75, rgb.green());
	/// assert_eq!(191, rgb.green_as_byte());
	/// assert_eq!(0.4980392157, rgb.blue());
	/// assert_eq!(127, rgb.blue_as_byte());
	/// assert_eq!(1.0, rgb.alpha());
	/// assert_eq!(255, rgb.alpha_as_byte());
	/// ```
	pub fn new(r: f32, g: f32, b: f32) -> Self { Color::new_alpha(r, g, b, 1.0) }
	
	/// Creates a new color using rgba with floating point numbers
	/// - **r**: The red channel to set
	/// - **g**: The green channel to set
	/// - **b**: The blue channel to set
	/// - **a**: The alpha channel to set
	/// 
	/// **Returns**: Returns a new color using rgba with floating point numbers
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let rgb = Color::new_alpha(0.5, 0.75, 0.4980392157, 0.00001);
	/// assert_eq!(0.5, rgb.red());
	/// assert_eq!(127, rgb.red_as_byte());
	/// assert_eq!(0.75, rgb.green());
	/// assert_eq!(191, rgb.green_as_byte());
	/// assert_eq!(0.4980392157, rgb.blue());
	/// assert_eq!(127, rgb.blue_as_byte());
	/// assert_eq!(0.00001, rgb.alpha());
	/// assert_eq!(0, rgb.alpha_as_byte());
	/// ```
	pub fn new_alpha(r: f32, g: f32, b: f32, a: f32) -> Self { Color {
		r: Math::clamp(r, 0.0, 1.0),
		g: Math::clamp(g, 0.0, 1.0),
		b: Math::clamp(b, 0.0, 1.0),
		a: Math::clamp(a, 0.0, 1.0),
	} }
	
	/// Creates a new color using rgba with bytes
	/// - **r**: The red channel to set
	/// - **g**: The green channel to set
	/// - **b**: The blue channel to set
	/// - **a**: The alpha channel to set
	/// 
	/// **Returns**: Returns a new color using rgba with bytes
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let rgb = Color::new_rgba(127, 20, 200, 50);
	/// assert_eq!(0.4980392157, rgb.red());
	/// assert_eq!(127, rgb.red_as_byte());
	/// assert_eq!(0.07843137255, rgb.green());
	/// assert_eq!(20, rgb.green_as_byte());
	/// assert_eq!(0.7843137255, rgb.blue());
	/// assert_eq!(200, rgb.blue_as_byte());
	/// assert_eq!(0.1960784314, rgb.alpha());
	/// assert_eq!(50, rgb.alpha_as_byte());
	/// ```
	pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
		Color::new_alpha(
			r as f32 / 255.0,
			g as f32 / 255.0,
			b as f32 / 255.0,
			a as f32 / 255.0
		)
	}
	
	/// Creates a new color using rgb with bytes
	/// - **r**: The red channel to set
	/// - **g**: The green channel to set
	/// - **b**: The blue channel to set
	/// 
	/// **Returns**: Returns a new color using rgb with bytes
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let rgb = Color::new_rgb(127, 20, 200);
	/// assert_eq!(0.4980392157, rgb.red());
	/// assert_eq!(127, rgb.red_as_byte());
	/// assert_eq!(0.07843137255, rgb.green());
	/// assert_eq!(20, rgb.green_as_byte());
	/// assert_eq!(0.7843137255, rgb.blue());
	/// assert_eq!(200, rgb.blue_as_byte());
	/// assert_eq!(1.0, rgb.alpha());
	/// assert_eq!(255, rgb.alpha_as_byte());
	/// ```
	pub fn new_rgb(r: u8, g: u8, b: u8) -> Self { Color::new_rgba(r, g, b, 255) }
	
	/// Creates a new color using either a known name (found on the [W3 site](https://www.w3schools.com/tags/ref_colornames.asp))
	/// or by use of a hex code (such as #5A9CA4 or #669). Hex codes can also include alpha values (such as #5A9CA4DD or #669D).
	/// - **name_or_hex**: The known name or hex code for the color. If this is invalid, it will return the color black.
	/// Typing in the known name is case-insensitive and ignores both spaces and underscores. So `olivedrab` is the same as `Olive Drab` or `olive_drab`.
	/// 
	/// **Returns**: Returns a new color using either a known name or hex code
	/// #### Remarks
	/// If you are using `no_std` and are creating a color from a known name, this library specifically avoids trying to allocate memory
	/// and as such the name must be all lowercases with no spaces or underscores whatsoever. So `olivedrab` is not the same as `Olive Drab` nor `olive_drab`.
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let tomato = Color::new_str("tomato");
	/// let expected = Color::new_rgb(255, 99, 71);
	/// assert_eq!(expected, tomato);
	/// ```
	pub fn new_str(name_or_hex: &str) -> Self {
		match from_known_name(name_or_hex) {
			Option::Some(color) => color,
			Option::None => Color::new(0.0, 0.0, 0.0),
		}
	}
}

/// Properties
impl Color {
	/// Gets the red channel for the color
	/// 
	/// **Returns**: Returns the red channel as a floating point number between 0.0 and 1.0
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let color = Color::new(0.345, 1.0, 1.0);
	/// assert_eq!(0.345, color.red());
	/// ```
	pub fn red(&self) -> f32 { self.r }
	
	/// Sets the red channel for the color
	/// - **value**: The value to set the red channel to
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let mut color = Color::new_str("tomato");
	/// color.set_red(0.1);
	/// assert_eq!(0.1, color.red());
	/// ```
	pub fn set_red(&mut self, value: f32) { self.r = Math::clamp(value, 0.0, 1.0); }
	
	/// Gets the red channel for the color
	/// 
	/// **Returns**: Returns the red channel as a byte
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let color = Color::new(0.345, 1.0, 1.0);
	/// assert_eq!(87, color.red_as_byte());
	/// ```
	pub fn red_as_byte(&self) -> u8 { (self.r * 255.0) as u8 }
	
	/// Sets the red channel for the color with a byte
	/// - **value**: The value to set the red channel to
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let mut color = Color::new_str("tomato");
	/// color.set_red_as_byte(25);
	/// assert_eq!(25, color.red_as_byte());
	/// ```
	pub fn set_red_as_byte(&mut self, value: u8) { self.r = value as f32 / 255.0 }
	
	/// Gets the green channel for the color
	/// 
	/// **Returns**: Returns the green channel as a floating point number between 0.0 and 1.0
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let color = Color::new(0.1, 0.9, 0.1);
	/// assert_eq!(0.9, color.green());
	/// ```
	pub fn green(&self) -> f32 { self.g }
	
	/// Sets the green channel for the color
	/// - **value**: The value to set the green channel to
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let mut color = Color::new_str("tomato");
	/// color.set_green(0.2);
	/// assert_eq!(0.2, color.green());
	/// ```
	pub fn set_green(&mut self, value: f32) { self.g = Math::clamp(value, 0.0, 1.0); }
	
	/// Gets the green channel for the color
	/// 
	/// **Returns**: Returns the green channel as a byte
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let color = Color::new(0.1, 0.9, 0.1);
	/// assert_eq!(229, color.green_as_byte());
	/// ```
	pub fn green_as_byte(&self) -> u8 { (self.g * 255.0) as u8 }
	
	/// Sets the green channel for the color using a byte
	/// - **value**: The value to set the green channel to
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let mut color = Color::new_str("tomato");
	/// color.set_green_as_byte(50);
	/// assert_eq!(50, color.green_as_byte());
	/// ```
	pub fn set_green_as_byte(&mut self, value: u8) { self.g = value as f32 / 255.0 }
	
	/// Gets the blue channel for the color
	/// 
	/// **Returns**: Returns the blue channel as a floating point number between 0.0 and 1.0
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let color = Color::new(0.1, 0.9, 0.1);
	/// assert_eq!(0.1, color.blue());
	/// ```
	pub fn blue(&self) -> f32 { self.b }
	
	/// Sets the blue channel for the color
	/// - **value**: The value to set the blue channel to
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let mut color = Color::new_str("tomato");
	/// color.set_blue(0.3);
	/// assert_eq!(0.3, color.blue());
	/// ```
	pub fn set_blue(&mut self, value: f32) { self.b = Math::clamp(value, 0.0, 1.0); }
	
	/// Gets the blue channel for the color
	/// 
	/// **Returns**: Returns the blue channel as a byte
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let color = Color::new(0.1, 0.9, 0.1);
	/// assert_eq!(25, color.blue_as_byte());
	/// ```
	pub fn blue_as_byte(&self) -> u8 { (self.b * 255.0) as u8 }
	
	/// Sets the blue channel for the color using a byte
	/// - **value**: The value to set the blue channel to
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let mut color = Color::new_str("tomato");
	/// color.set_blue_as_byte(150);
	/// assert_eq!(150, color.blue_as_byte());
	/// ```
	pub fn set_blue_as_byte(&mut self, value: u8) { self.b = value as f32 / 255.0 }
	
	/// Gets the alpha channel for the color
	/// 
	/// **Returns**: Returns the alpha channel as a floating point number between 0.0 and 1.0
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let color = Color::new_alpha(1.0, 1.0, 1.0, 0.4);
	/// assert_eq!(0.4, color.alpha());
	/// ```
	pub fn alpha(&self) -> f32 { self.a }
	
	/// Sets the alpha channel for the color
	/// - **value**: The value to set the alpha channel to
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let mut color = Color::new_str("tomato");
	/// color.set_alpha(0.4);
	/// assert_eq!(0.4, color.alpha());
	/// ```
	pub fn set_alpha(&mut self, value: f32) { self.a = Math::clamp(value, 0.0, 1.0); }
	
	/// Gets the alpha channel for the color
	/// 
	/// **Returns**: Returns the alpha channel as a byte
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let color = Color::new_alpha(1.0, 1.0, 1.0, 0.4);
	/// assert_eq!(102, color.alpha_as_byte());
	/// ```
	pub fn alpha_as_byte(&self) -> u8 { (self.a * 255.0) as u8 }
	
	/// Sets the alpha channel for the color using a byte
	/// - **value**: The value to set the alpha channel to
	/// #### Examples
	/// ```
	/// # use mathx::Color;
	/// let mut color = Color::new_str("tomato");
	/// color.set_alpha_as_byte(200);
	/// assert_eq!(200, color.alpha_as_byte());
	/// ```
	pub fn set_alpha_as_byte(&mut self, value: u8) { self.a = value as f32 / 255.0 }
}

// Equates
impl Eq for Color {}
impl PartialEq for Color {
	fn eq(&self, other: &Self) -> bool {
		Math::approx(self.r, other.r)
		&& Math::approx(self.g, other.g)
		&& Math::approx(self.b, other.b)
		&& Math::approx(self.a, other.a)
	}
}

// Display
#[cfg(not(feature = "no_std"))]
impl std::fmt::Display for Color {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("({}, {}, {}, {})", self.r, self.g, self.b, self.a))
	}
}

fn from_hex(hex: &str) -> Option<Color> {
	if !hex.starts_with("#") { return Option::None; }
	
	let mut red = 0u8;
	let mut green = 0u8;
	let mut blue = 0u8;
	let mut alpha = 255u8;
	
	if hex.len() == 4 || hex.len() == 5 {
		
		red = match get_byte_from_doubled_hex(&hex[1..2]) {
			Result::Err(_) => return Option::None,
			Result::Ok(value) => value,
		};
		green = match get_byte_from_doubled_hex(&hex[2..3]) {
			Result::Err(_) => return Option::None,
			Result::Ok(value) => value,
		};
		blue = match get_byte_from_doubled_hex(&hex[3..4]) {
			Result::Err(_) => return Option::None,
			Result::Ok(value) => value,
		};
	}
	if hex.len() == 5 {
		alpha = match get_byte_from_doubled_hex(&hex[4..5]) {
			Result::Err(_) => return Option::None,
			Result::Ok(value) => value,
		};
	}
	if hex.len() == 7 || hex.len() == 9 {
		red = match u8::from_str_radix(&hex[1..3], 16) {
			Result::Err(_) => return Option::None,
			Result::Ok(value) => value,
		};
		green = match u8::from_str_radix(&hex[3..5], 16) {
			Result::Err(_) => return Option::None,
			Result::Ok(value) => value,
		};
		blue = match u8::from_str_radix(&hex[5..7], 16) {
			Result::Err(_) => return Option::None,
			Result::Ok(value) => value,
		};
	}
	if hex.len() == 9 {
		alpha = match u8::from_str_radix(&hex[7..9], 16) {
			Result::Err(_) => return Option::None,
			Result::Ok(value) => value,
		};
	}
	
	Option::Some(Color::new_rgba(red, green, blue, alpha))
}

fn get_byte_from_doubled_hex(hex: &str) -> Result<u8, ()> {
	let num = match u8::from_str_radix(hex, 16) {
		Result::Err(_) => return Result::Err(()),
		Result::Ok(value) => value,
	};
	return Result::Ok(num * 16 + num);
}

#[cfg(feature = "no_std")]
fn from_known_name(name: &str) -> Option<Color> {
	match name {
		"aliceblue" => from_hex("#F0F8FF"),
		"antiquewhite" => from_hex("#FAEBD7"),
		"aqua" => from_hex("#00FFFF"),
		"aquamarine" => from_hex("#7FFFD4"),
		"azure" => from_hex("#F0FFFF"),
		"beige" => from_hex("#F5F5DC"),
		"bisque" => from_hex("#FFE4C4"),
		"black" => from_hex("#000000"),
		"blanchedalmond" => from_hex("#FFEBCD"),
		"blue" => from_hex("#0000FF"),
		"blueviolet" => from_hex("#8A2BE2"),
		"brown" => from_hex("#A52A2A"),
		"burlywood" => from_hex("#DEB887"),
		"cadetblue" => from_hex("#5F9EA0"),
		"chartreuse" => from_hex("#7FFF00"),
		"chocolate" => from_hex("#D2691E"),
		"coral" => from_hex("#FF7F50"),
		"cornflowerblue" => from_hex("#6495ED"),
		"cornsilk" => from_hex("#FFF8DC"),
		"crimson" => from_hex("#DC143C"),
		"cyan" => from_hex("#00FFFF"),
		"darkblue" => from_hex("#00008B"),
		"darkcyan" => from_hex("#008B8B"),
		"darkgoldenrod" => from_hex("#B8860B"),
		"darkgray" => from_hex("#A9A9A9"),
		"darkgrey" => from_hex("#A9A9A9"),
		"darkgreen" => from_hex("#006400"),
		"darkkhaki" => from_hex("#BDB76B"),
		"darkmagenta" => from_hex("#8B008B"),
		"darkolivegreen" => from_hex("#556B2F"),
		"darkorange" => from_hex("#FF8C00"),
		"darkorchid" => from_hex("#9932CC"),
		"darkred" => from_hex("#8B0000"),
		"darksalmon" => from_hex("#E9967A"),
		"darkseagreen" => from_hex("#8FBC8F"),
		"darkslateblue" => from_hex("#483D8B"),
		"darkslategray" => from_hex("#2F4F4F"),
		"darkslategrey" => from_hex("#2F4F4F"),
		"darkturquoise" => from_hex("#00CED1"),
		"darkviolet" => from_hex("#9400D3"),
		"deeppink" => from_hex("#FF1493"),
		"deepskyblue" => from_hex("#00BFFF"),
		"dimgray" => from_hex("#696969"),
		"dimgrey" => from_hex("#696969"),
		"dodgerblue" => from_hex("#1E90FF"),
		"firebrick" => from_hex("#B22222"),
		"floralwhite" => from_hex("#FFFAF0"),
		"forestgreen" => from_hex("#228B22"),
		"fuchsia" => from_hex("#FF00FF"),
		"gainsboro" => from_hex("#DCDCDC"),
		"ghostwhite" => from_hex("#F8F8FF"),
		"gold" => from_hex("#FFD700"),
		"goldenrod" => from_hex("#DAA520"),
		"gray" => from_hex("#808080"),
		"grey" => from_hex("#808080"),
		"green" => from_hex("#008000"),
		"greenyellow" => from_hex("#ADFF2F"),
		"honeydew" => from_hex("#F0FFF0"),
		"hotpink" => from_hex("#FF69B4"),
		"indianred" => from_hex("#CD5C5C"),
		"indigo" => from_hex("#4B0082"),
		"ivory" => from_hex("#FFFFF0"),
		"khaki" => from_hex("#F0E68C"),
		"lavender" => from_hex("#E6E6FA"),
		"lavenderblush" => from_hex("#FFF0F5"),
		"lawngreen" => from_hex("#7CFC00"),
		"lemonchiffon" => from_hex("#FFFACD"),
		"lightblue" => from_hex("#ADD8E6"),
		"lightcoral" => from_hex("#F08080"),
		"lightcyan" => from_hex("#E0FFFF"),
		"lightgoldenrodyellow" => from_hex("#FAFAD2"),
		"lightgray" => from_hex("#D3D3D3"),
		"lightgrey" => from_hex("#D3D3D3"),
		"lightgreen" => from_hex("#90EE90"),
		"lightpink" => from_hex("#FFB6C1"),
		"lightsalmon" => from_hex("#FFA07A"),
		"lightseagreen" => from_hex("#20B2AA"),
		"lightskyblue" => from_hex("#87CEFA"),
		"lightslategray" => from_hex("#778899"),
		"lightslategrey" => from_hex("#778899"),
		"lightsteelblue" => from_hex("#B0C4DE"),
		"lightyellow" => from_hex("#FFFFE0"),
		"lime" => from_hex("#00FF00"),
		"limegreen" => from_hex("#32CD32"),
		"linen" => from_hex("#FAF0E6"),
		"magenta" => from_hex("#FF00FF"),
		"maroon" => from_hex("#800000"),
		"mediumaquamarine" => from_hex("#66CDAA"),
		"mediumblue" => from_hex("#0000CD"),
		"mediumorchid" => from_hex("#BA55D3"),
		"mediumpurple" => from_hex("#9370DB"),
		"mediumseagreen" => from_hex("#3CB371"),
		"mediumslateblue" => from_hex("#7B68EE"),
		"mediumspringgreen" => from_hex("#00FA9A"),
		"mediumturquoise" => from_hex("#48D1CC"),
		"mediumvioletred" => from_hex("#C71585"),
		"midnightblue" => from_hex("#191970"),
		"mintcream" => from_hex("#F5FFFA"),
		"mistyrose" => from_hex("#FFE4E1"),
		"moccasin" => from_hex("#FFE4B5"),
		"navajowhite" => from_hex("#FFDEAD"),
		"navy" => from_hex("#000080"),
		"oldlace" => from_hex("#FDF5E6"),
		"olive" => from_hex("#808000"),
		"olivedrab" => from_hex("#6B8E23"),
		"orange" => from_hex("#FFA500"),
		"orangered" => from_hex("#FF4500"),
		"orchid" => from_hex("#DA70D6"),
		"palegoldenrod" => from_hex("#EEE8AA"),
		"palegreen" => from_hex("#98FB98"),
		"paleturquoise" => from_hex("#AFEEEE"),
		"palevioletred" => from_hex("#DB7093"),
		"papayawhip" => from_hex("#FFEFD5"),
		"peachpuff" => from_hex("#FFDAB9"),
		"peru" => from_hex("#CD853F"),
		"pink" => from_hex("#FFC0CB"),
		"plum" => from_hex("#DDA0DD"),
		"powderblue" => from_hex("#B0E0E6"),
		"purple" => from_hex("#800080"),
		"rebeccapurple" => from_hex("#663399"),
		"red" => from_hex("#FF0000"),
		"rosybrown" => from_hex("#BC8F8F"),
		"royalblue" => from_hex("#4169E1"),
		"saddlebrown" => from_hex("#8B4513"),
		"salmon" => from_hex("#FA8072"),
		"sandybrown" => from_hex("#F4A460"),
		"seagreen" => from_hex("#2E8B57"),
		"seashell" => from_hex("#FFF5EE"),
		"sienna" => from_hex("#A0522D"),
		"silver" => from_hex("#C0C0C0"),
		"skyblue" => from_hex("#87CEEB"),
		"slateblue" => from_hex("#6A5ACD"),
		"slategray" => from_hex("#708090"),
		"slategrey" => from_hex("#708090"),
		"snow" => from_hex("#FFFAFA"),
		"springgreen" => from_hex("#00FF7F"),
		"steelblue" => from_hex("#4682B4"),
		"tan" => from_hex("#D2B48C"),
		"teal" => from_hex("#008080"),
		"thistle" => from_hex("#D8BFD8"),
		"tomato" => from_hex("#FF6347"),
		"turquoise" => from_hex("#40E0D0"),
		"violet" => from_hex("#EE82EE"),
		"wheat" => from_hex("#F5DEB3"),
		"white" => from_hex("#FFFFFF"),
		"whitesmoke" => from_hex("#F5F5F5"),
		"yellow" => from_hex("#FFFF00"),
		"yellowgreen" => from_hex("#9ACD32"),
		_ => from_hex(name),
	}
}

#[cfg(not(feature = "no_std"))]
fn from_known_name(name: &str) -> Option<Color> {
	match name.to_lowercase().replace(" ", "").replace("_", "").as_str() {
		"aliceblue" => from_hex("#F0F8FF"),
		"antiquewhite" => from_hex("#FAEBD7"),
		"aqua" => from_hex("#00FFFF"),
		"aquamarine" => from_hex("#7FFFD4"),
		"azure" => from_hex("#F0FFFF"),
		"beige" => from_hex("#F5F5DC"),
		"bisque" => from_hex("#FFE4C4"),
		"black" => from_hex("#000000"),
		"blanchedalmond" => from_hex("#FFEBCD"),
		"blue" => from_hex("#0000FF"),
		"blueviolet" => from_hex("#8A2BE2"),
		"brown" => from_hex("#A52A2A"),
		"burlywood" => from_hex("#DEB887"),
		"cadetblue" => from_hex("#5F9EA0"),
		"chartreuse" => from_hex("#7FFF00"),
		"chocolate" => from_hex("#D2691E"),
		"coral" => from_hex("#FF7F50"),
		"cornflowerblue" => from_hex("#6495ED"),
		"cornsilk" => from_hex("#FFF8DC"),
		"crimson" => from_hex("#DC143C"),
		"cyan" => from_hex("#00FFFF"),
		"darkblue" => from_hex("#00008B"),
		"darkcyan" => from_hex("#008B8B"),
		"darkgoldenrod" => from_hex("#B8860B"),
		"darkgray" => from_hex("#A9A9A9"),
		"darkgrey" => from_hex("#A9A9A9"),
		"darkgreen" => from_hex("#006400"),
		"darkkhaki" => from_hex("#BDB76B"),
		"darkmagenta" => from_hex("#8B008B"),
		"darkolivegreen" => from_hex("#556B2F"),
		"darkorange" => from_hex("#FF8C00"),
		"darkorchid" => from_hex("#9932CC"),
		"darkred" => from_hex("#8B0000"),
		"darksalmon" => from_hex("#E9967A"),
		"darkseagreen" => from_hex("#8FBC8F"),
		"darkslateblue" => from_hex("#483D8B"),
		"darkslategray" => from_hex("#2F4F4F"),
		"darkslategrey" => from_hex("#2F4F4F"),
		"darkturquoise" => from_hex("#00CED1"),
		"darkviolet" => from_hex("#9400D3"),
		"deeppink" => from_hex("#FF1493"),
		"deepskyblue" => from_hex("#00BFFF"),
		"dimgray" => from_hex("#696969"),
		"dimgrey" => from_hex("#696969"),
		"dodgerblue" => from_hex("#1E90FF"),
		"firebrick" => from_hex("#B22222"),
		"floralwhite" => from_hex("#FFFAF0"),
		"forestgreen" => from_hex("#228B22"),
		"fuchsia" => from_hex("#FF00FF"),
		"gainsboro" => from_hex("#DCDCDC"),
		"ghostwhite" => from_hex("#F8F8FF"),
		"gold" => from_hex("#FFD700"),
		"goldenrod" => from_hex("#DAA520"),
		"gray" => from_hex("#808080"),
		"grey" => from_hex("#808080"),
		"green" => from_hex("#008000"),
		"greenyellow" => from_hex("#ADFF2F"),
		"honeydew" => from_hex("#F0FFF0"),
		"hotpink" => from_hex("#FF69B4"),
		"indianred" => from_hex("#CD5C5C"),
		"indigo" => from_hex("#4B0082"),
		"ivory" => from_hex("#FFFFF0"),
		"khaki" => from_hex("#F0E68C"),
		"lavender" => from_hex("#E6E6FA"),
		"lavenderblush" => from_hex("#FFF0F5"),
		"lawngreen" => from_hex("#7CFC00"),
		"lemonchiffon" => from_hex("#FFFACD"),
		"lightblue" => from_hex("#ADD8E6"),
		"lightcoral" => from_hex("#F08080"),
		"lightcyan" => from_hex("#E0FFFF"),
		"lightgoldenrodyellow" => from_hex("#FAFAD2"),
		"lightgray" => from_hex("#D3D3D3"),
		"lightgrey" => from_hex("#D3D3D3"),
		"lightgreen" => from_hex("#90EE90"),
		"lightpink" => from_hex("#FFB6C1"),
		"lightsalmon" => from_hex("#FFA07A"),
		"lightseagreen" => from_hex("#20B2AA"),
		"lightskyblue" => from_hex("#87CEFA"),
		"lightslategray" => from_hex("#778899"),
		"lightslategrey" => from_hex("#778899"),
		"lightsteelblue" => from_hex("#B0C4DE"),
		"lightyellow" => from_hex("#FFFFE0"),
		"lime" => from_hex("#00FF00"),
		"limegreen" => from_hex("#32CD32"),
		"linen" => from_hex("#FAF0E6"),
		"magenta" => from_hex("#FF00FF"),
		"maroon" => from_hex("#800000"),
		"mediumaquamarine" => from_hex("#66CDAA"),
		"mediumblue" => from_hex("#0000CD"),
		"mediumorchid" => from_hex("#BA55D3"),
		"mediumpurple" => from_hex("#9370DB"),
		"mediumseagreen" => from_hex("#3CB371"),
		"mediumslateblue" => from_hex("#7B68EE"),
		"mediumspringgreen" => from_hex("#00FA9A"),
		"mediumturquoise" => from_hex("#48D1CC"),
		"mediumvioletred" => from_hex("#C71585"),
		"midnightblue" => from_hex("#191970"),
		"mintcream" => from_hex("#F5FFFA"),
		"mistyrose" => from_hex("#FFE4E1"),
		"moccasin" => from_hex("#FFE4B5"),
		"navajowhite" => from_hex("#FFDEAD"),
		"navy" => from_hex("#000080"),
		"oldlace" => from_hex("#FDF5E6"),
		"olive" => from_hex("#808000"),
		"olivedrab" => from_hex("#6B8E23"),
		"orange" => from_hex("#FFA500"),
		"orangered" => from_hex("#FF4500"),
		"orchid" => from_hex("#DA70D6"),
		"palegoldenrod" => from_hex("#EEE8AA"),
		"palegreen" => from_hex("#98FB98"),
		"paleturquoise" => from_hex("#AFEEEE"),
		"palevioletred" => from_hex("#DB7093"),
		"papayawhip" => from_hex("#FFEFD5"),
		"peachpuff" => from_hex("#FFDAB9"),
		"peru" => from_hex("#CD853F"),
		"pink" => from_hex("#FFC0CB"),
		"plum" => from_hex("#DDA0DD"),
		"powderblue" => from_hex("#B0E0E6"),
		"purple" => from_hex("#800080"),
		"rebeccapurple" => from_hex("#663399"),
		"red" => from_hex("#FF0000"),
		"rosybrown" => from_hex("#BC8F8F"),
		"royalblue" => from_hex("#4169E1"),
		"saddlebrown" => from_hex("#8B4513"),
		"salmon" => from_hex("#FA8072"),
		"sandybrown" => from_hex("#F4A460"),
		"seagreen" => from_hex("#2E8B57"),
		"seashell" => from_hex("#FFF5EE"),
		"sienna" => from_hex("#A0522D"),
		"silver" => from_hex("#C0C0C0"),
		"skyblue" => from_hex("#87CEEB"),
		"slateblue" => from_hex("#6A5ACD"),
		"slategray" => from_hex("#708090"),
		"slategrey" => from_hex("#708090"),
		"snow" => from_hex("#FFFAFA"),
		"springgreen" => from_hex("#00FF7F"),
		"steelblue" => from_hex("#4682B4"),
		"tan" => from_hex("#D2B48C"),
		"teal" => from_hex("#008080"),
		"thistle" => from_hex("#D8BFD8"),
		"tomato" => from_hex("#FF6347"),
		"turquoise" => from_hex("#40E0D0"),
		"violet" => from_hex("#EE82EE"),
		"wheat" => from_hex("#F5DEB3"),
		"white" => from_hex("#FFFFFF"),
		"whitesmoke" => from_hex("#F5F5F5"),
		"yellow" => from_hex("#FFFF00"),
		"yellowgreen" => from_hex("#9ACD32"),
		_ => from_hex(name),
	}
}
