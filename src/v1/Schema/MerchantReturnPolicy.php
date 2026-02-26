<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class MerchantReturnPolicy extends TypedSchema {
	public const A_SCHEMA_TYPE = 'MerchantReturnPolicy';

	public function __construct(
		public string|array $applicableCountry,
		public MerchantReturnEnumeration $returnPolicyCategory,
		public null|int $merchantReturnDays = null,
		public null|string $merchantReturnLink = null,
		public null|ReturnMethodEnumeration $returnMethod = null,
		public null|ReturnFeesEnumeration $returnFees = null,
		public null|MonetaryAmount $returnShippingFeesAmount = null,
		public null|RefundTypeEnumeration $refundType = null,
		public null|OfferItemCondition $itemCondition = null,
		public null|ReturnLabelSourceEnumeration $returnLabelSource = null,
		public null|string $returnPolicyCountry = null,
		public null|MonetaryAmount|float $restockingFee = null,
		public null|ReturnFeesEnumeration $customerRemorseReturnFees = null,
		public null|ReturnLabelSourceEnumeration $customerRemorseReturnLabelSource = null,
		public null|MonetaryAmount $customerRemorseReturnShippingFeesAmount = null,
		public null|ReturnFeesEnumeration $itemDefectReturnFees = null,
		public null|ReturnLabelSourceEnumeration $itemDefectReturnLabelSource = null,
		public null|MonetaryAmount $itemDefectReturnShippingFeesAmount = null,
		public null|MerchantReturnPolicySeasonalOverride|array $returnPolicySeasonalOverride = null,
	) {}
}
