<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class OpeningHoursSpecification extends TypedSchema {
	public const A_SCHEMA_TYPE = 'OpeningHoursSpecification';

	public function __construct(
		public null|DayOfWeek $dayOfWeek = null,
		public null|string $opens = null,
		public null|string $closes = null,
		public null|string $validFrom = null,
		public null|string $validThrough = null,
	) {}
}
