<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class PeopleAudience extends TypedSchema {
	public const A_SCHEMA_TYPE = 'PeopleAudience';

	public function __construct(
		public null|string $suggestedGender = null,
		public null|int|float $suggestedMinAge = null,
		public null|int|float $suggestedMaxAge = null,
	) {}
}
