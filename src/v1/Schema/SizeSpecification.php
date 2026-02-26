<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class SizeSpecification extends TypedSchema {
	public const A_SCHEMA_TYPE = 'SizeSpecification';

	public function __construct(
		public string $name,
		public null|string $sizeGroup = null,
		public null|string $sizeSystem = null,
	) {}
}
