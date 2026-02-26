<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ItemList extends TypedSchema {
	public const A_SCHEMA_TYPE = 'ItemList';

	/**
	 * @param ListItem[] $itemListElement
	 */
	public function __construct(
		public array $itemListElement,
		public null|string $itemListOrder = null,
		public null|int $numberOfItems = null,
	) {}
}
