<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class GeoCoordinates extends TypedSchema {
	public const A_SCHEMA_TYPE = 'GeoCoordinates';

	public function __construct(
		public float $latitude,
		public float $longitude,
	) {}
}
