<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ListItem extends TypedSchema {
	const A_SCHEMA_TYPE = 'ListItem';

	public function __construct(
		public int $position,
		public string $name,
		public null|string $item = null,
	) {

	}
}
