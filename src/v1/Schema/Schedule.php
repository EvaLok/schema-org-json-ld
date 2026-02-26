<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Schedule extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Schedule';

	public function __construct(
		public string $repeatFrequency,
		public null|int $repeatCount = null,
		public null|string $startDate = null,
		public null|string $endDate = null,
	) {}
}
