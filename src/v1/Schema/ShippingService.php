<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\Enum\FulfillmentTypeEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ShippingService extends TypedSchema {
	public const A_SCHEMA_TYPE = 'ShippingService';

	public function __construct(
		/** @var ShippingConditions|ShippingConditions[] $shippingConditions */
		public ShippingConditions|array $shippingConditions,
		public null|string $name = null,
		public null|string $description = null,
		public null|FulfillmentTypeEnumeration $fulfillmentType = null,
		public null|ServicePeriod $handlingTime = null,
		public null|MemberProgramTier $validForMemberTier = null,
	) {}
}
