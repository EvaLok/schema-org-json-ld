<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class PropertyValue extends TypedSchema {
	public const A_SCHEMA_TYPE = 'PropertyValue';

	public function __construct(
		public string $name,
		public string $value,
	) {}
}
