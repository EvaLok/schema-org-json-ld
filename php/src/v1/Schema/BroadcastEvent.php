<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class BroadcastEvent extends TypedSchema {
	public const A_SCHEMA_TYPE = 'BroadcastEvent';

	public function __construct(
		public bool $isLiveBroadcast,
		public null|string $startDate = null,
		public null|string $endDate = null,
	) {}
}
