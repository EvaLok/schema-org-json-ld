<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum RefundTypeEnumeration: string {
	case ExchangeRefund = 'https://schema.org/ExchangeRefund';
	case FullRefund = 'https://schema.org/FullRefund';
	case StoreCreditRefund = 'https://schema.org/StoreCreditRefund';
}
