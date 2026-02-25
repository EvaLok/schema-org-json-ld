<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class LocationFeatureSpecification extends TypedSchema {
	public const A_SCHEMA_TYPE = 'LocationFeatureSpecification';

	public function __construct(
		public string $name,
		public bool|string $value,
	) {}
}
