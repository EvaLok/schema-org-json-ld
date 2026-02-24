<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Brand extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Brand';

	public function __construct(
		public string $name,
		public null|string $description = null,
	) {}
}
