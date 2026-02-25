<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ListItem extends TypedSchema {
	public const A_SCHEMA_TYPE = 'ListItem';

	public function __construct(
		public int $position,
		public null|string $name = null,
		public null|string|TypedSchema $item = null,
		public null|string $url = null,
	) {}
}
