use bevy::ecs::query::QueryData;
use bevy::ecs::query::QueryFilter;
use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use std::iter::Chain;



#[extend::ext(name=HierarchyQueryExtExt)]
pub impl<
		'w,
		's,
		D: QueryData,
		F: QueryFilter,
		T: HierarchyQueryExt<'w, 's, D, F>,
	> T
{
	/// Iterates over all ancestors of the given entity, including the entity itself.
	fn iter_ancestors_inclusive(
		&'w self,
		entity: Entity,
	) -> Chain<
		std::iter::Once<bevy::prelude::Entity>,
		bevy::prelude::AncestorIter<'w, 's, D, F>,
	>
	where
		D::ReadOnly: WorldQuery<Item<'w> = &'w Parent>,
	{
		Iterator::chain(std::iter::once(entity), self.iter_ancestors(entity))
	}

	/// Iterates over all descendants of the given entity, including the entity itself.
	fn iter_descendants_inclusive(
		&'w self,
		entity: Entity,
	) -> Chain<
		std::iter::Once<bevy::prelude::Entity>,
		bevy::prelude::DescendantIter<'w, 's, D, F>,
	>
	where
		D::ReadOnly: WorldQuery<Item<'w> = &'w Children>,
	{
		Iterator::chain(std::iter::once(entity), self.iter_descendants(entity))
	}
}
