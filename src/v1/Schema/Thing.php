<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Thing extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Thing';

	public function __construct(
		public string $name,
	) {}
}
