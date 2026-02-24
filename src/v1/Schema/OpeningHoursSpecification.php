<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class OpeningHoursSpecification extends TypedSchema {
	public const A_SCHEMA_TYPE = 'OpeningHoursSpecification';

	public function __construct(
		public DayOfWeek $dayOfWeek,
		public string $opens,
		public string $closes,
		public null|string $validFrom = null,
		public null|string $validThrough = null,
	) {}
}
