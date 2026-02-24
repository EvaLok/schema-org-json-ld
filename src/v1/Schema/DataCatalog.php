<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class DataCatalog extends TypedSchema {
	public const A_SCHEMA_TYPE = 'DataCatalog';

	public function __construct(
		public string $name,
	) {}
}
