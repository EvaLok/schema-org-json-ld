<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Rating extends TypedSchema {

	const A_SCHEMA_TYPE = 'Rating';

	public function __construct(
		public float|int $ratingValue,
		public null|float|int $bestRating = null,
		public null|float|int $worstRating = null,
	) {

	}
}
