use crate::prelude::*;
use sweet_core::prelude::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::Event;

/**
The sweet loader is responsible for gluing together the hydrated events and blocks with
the dom.

The core of the event system is a global object called `_sweet`.
All events will look something like `onclick="_sweet.event(0,event)"`, where the first argument is the index in which the parser found the event block. Ids may skip numbers if a conditional block, rsx fragments, etc are found.

A tiny shim (111 bytes) is used to collect all events that are emmitted while the wasm is loading.

On initialization all of these events will be played back in order, and `_sweet` will directly call the handlers with corresponding id.

**/
#[derive(Default)]
pub struct SweetLoader {
	pub postion_visitor: RsxTreePositionVisitor,
}

impl RsxTreeVisitorOwned<RustParts> for SweetLoader {
	fn visit_node(
		&mut self,
		mut node: Node<RustParts>,
	) -> ParseResult<Option<Vec<Node<RustParts>>>> {
		self.postion_visitor.visit_node(&node)?;
		let children = node.take_children();
		match &node {
			Node::Doctype => {}
			Node::Comment(_) => {}
			Node::Element(element) => todo!(),
			Node::Text(_) => {}
			Node::TextBlock(text_block) => {
				// self.visit_text_block(blocks, recv);
			}
			Node::Component(_, vec) => todo!(),
		}
		self.postion_visitor.leave_node(&node)?;
		Ok(children)
	}

	fn leave_node(&mut self) -> ParseResult<()> { Ok(()) }

	fn visit_children(
		&mut self,
		children: Vec<Node<RustParts>>,
	) -> ParseResult<Vec<Node<RustParts>>> {
		Ok(children)
	}

	fn leave_children(&mut self) -> ParseResult<()> { Ok(()) }
}


impl SweetLoader {
	pub fn load(self, rsx: impl Rsx) -> ParseResult<()> {
		console_error_panic_hook::set_once();

		let rsx = rsx.into_rsx_tree();
		for _item in rsx.nodes.into_iter() {
			// match item {
			// 	RustParts::DynNodeId => todo!(),
			// 	RustParts::TextBlock(_) => todo!(),
			// 	RustParts::AttributeBlock(_) => todo!(),
			// 	RustParts::AttributeValue(_) => todo!(),
			// 	RustParts::Event(fn_mut) => todo!(),
			// 	RustParts::Component(rsx_parts) => todo!(),
			// }
		}
		// let (send, recv) = flume::unbounded();
		// let Hydrated { events, blocks } = page.hydrate(send)?;
		// self.handle_events(events)?;
		// self.handle_blocks(blocks, recv)?;
		Ok(())
	}


	pub fn visit_text_block(
		&self,
		blocks: Vec<HydratedTextBlock>,
		recv: flume::Receiver<(usize, String)>,
	) -> ParseResult<()> {
		// sparse array
		let mut live_blocks = Vec::<Option<LiveBlock>>::new();


		wasm_bindgen_futures::spawn_local(async move {
			while let Ok(block_event) = recv.recv_async().await {
				let block = blocks.get(block_event.0).expect(&format!(
					"failed to get hydrated block with id: {}",
					block_event.0
				));
				let block_str = block_event.1;

				let live_block = {
					if let Some(Some(block)) =
						live_blocks.get_mut(block.node_id)
					{
						block
					} else {
						let live_block = LiveBlock::new(block).unwrap();

						if live_blocks.len() <= block.node_id {
							live_blocks.resize_with(block.node_id + 1, || None);
						}

						live_blocks[block.node_id] = Some(live_block);
						live_blocks
							.get_mut(block.node_id)
							.unwrap()
							.as_mut()
							.unwrap()
					}
				};

				live_block.parts[block.part_index] = block_str;
				let inner_html = live_block.parts.join("");
				live_block.el.set_inner_html(&inner_html);
			}
		});
		Ok(())
	}
	pub fn handle_events(
		&self,
		mut events: Vec<HydratedEvent>,
	) -> ParseResult<()> {
		let mut func = move |id: usize, evt: Event| {
			let handler = events.get_mut(id).expect(&format!(
				"failed to get hydrated event with id: {}",
				id
			));
			handler(evt);
		};


		sweet_loader_extern::GLOBAL.with(|global| {
			let sweet = match js_sys::Reflect::get(&global, &"_sweet".into()) {
				Ok(s) if !s.is_undefined() => s,
				_ => {
					let obj = js_sys::Object::new();

					js_sys::Reflect::set(&global, &"_sweet".into(), &obj)
						.unwrap_or_default();
					obj.into()
				}
			};
			if let Ok(uncanny) = js_sys::Reflect::get(&sweet, &"uncanny".into())
			{
				let uncanny_array = js_sys::Array::from(&uncanny);
				for item in uncanny_array.iter() {
					let event_arr = js_sys::Array::from(&item);
					if event_arr.length() == 2 {
						let id =
							event_arr.get(0).as_f64().unwrap_or(0.0) as usize;
						let event = web_sys::Event::from(event_arr.get(1));
						func(id, event);
					}
				}
				js_sys::Reflect::delete_property(
					&sweet.unchecked_ref(),
					&"uncanny".into(),
				)
				.unwrap_or_default();
			}
			let closure =
				Closure::wrap(Box::new(move |id: usize, evt: web_sys::Event| {
					func(id, evt);
				}) as Box<dyn FnMut(usize, web_sys::Event)>);

			js_sys::Reflect::set(&sweet, &"event".into(), &closure.as_ref())
				.unwrap_or_default();

			closure.forget();
		});
		Ok(())
	}
}



pub mod sweet_loader_extern {
	use wasm_bindgen::prelude::*;
	#[wasm_bindgen]
	extern "C" {
		#[wasm_bindgen]
		#[wasm_bindgen(thread_local_v2,js_name = globalThis)]
		pub static GLOBAL: JsValue;
	}
}
