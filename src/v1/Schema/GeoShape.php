<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class GeoShape extends TypedSchema {
	public const A_SCHEMA_TYPE = 'GeoShape';

	public function __construct(
		public null|string $box = null,
	) {}
}
