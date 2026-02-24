<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class AggregateRating extends TypedSchema {

	const A_SCHEMA_TYPE = 'AggregateRating';

	public function __construct(
		public float|int $ratingValue,
		public null|float|int $bestRating = null,
		public null|float|int $worstRating = null,
		public null|int $ratingCount = null,
		public null|int $reviewCount = null,
	) {

	}
}
