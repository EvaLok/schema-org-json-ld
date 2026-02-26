<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class MerchantReturnPolicySeasonalOverride extends TypedSchema {
	public const A_SCHEMA_TYPE = 'MerchantReturnPolicySeasonalOverride';

	public function __construct(
		public string $startDate,
		public string $endDate,
		public MerchantReturnEnumeration $returnPolicyCategory,
		public null|int $merchantReturnDays = null,
	) {}
}
