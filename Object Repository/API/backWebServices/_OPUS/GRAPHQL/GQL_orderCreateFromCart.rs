<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GQL_orderCreateFromCart</name>
   <tag></tag>
   <elementGuidId>d060ace5-63b6-4688-b57b-c6a17eca31b6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;operationName\&quot;: \&quot;CreateOrderFromCart\&quot;,\n  \&quot;variables\&quot;: {\n    \&quot;command\&quot;: { \&quot;cartId\&quot;: \&quot;${cartId}\&quot; }\n  },\n  \&quot;query\&quot;: \&quot;mutation CreateOrderFromCart($command: InputCreateOrderFromCartType!) {\\n  createOrderFromCart(command: $command) {\\n    ...fullOrderFields\\n  }\\n}\\n\\nfragment fullOrderFields on OpusCustomerOrderType {\\n  ...shortOrderFields\\n  comment\\n  purchaseOrderNumber\\n  currency {\\n    ...currencyFields\\n  }\\n  shipmentItems {\\n    status\\n    quantity\\n    sku\\n    subOrderNumber\\n    comment\\n    price {\\n      ...moneyFields\\n    }\\n    updatedPrice {\\n      ...moneyFields\\n    }\\n    lineItem {\\n      name\\n    }\\n    supplier {\\n      name\\n    }\\n  }\\n  shipments {\\n    id\\n    shipmentMethodCode\\n    shipmentMethodOption\\n    shippingMethod {\\n      logoUrl\\n      typeName\\n    }\\n    price {\\n      ...moneyFields\\n    }\\n    deliveryAddress {\\n      ...orderAddressFields\\n    }\\n    comment\\n  }\\n  inPayments(first: 1, sort: \\\&quot;CreatedDate:desc\\\&quot;) {\\n    id\\n    number\\n    isApproved\\n    gatewayCode\\n    status\\n    paymentMethod {\\n      logoUrl\\n      code\\n      typeName\\n      paymentMethodType\\n      paymentMethodGroupType\\n    }\\n    billingAddress {\\n      ...orderAddressFields\\n    }\\n    comment\\n    dynamicProperties {\\n      name\\n      value\\n    }\\n  }\\n  availablePaymentMethods {\\n    code\\n    logoUrl\\n    price {\\n      ...moneyFields\\n    }\\n    settings {\\n      key\\n      value\\n    }\\n  }\\n  updatedTotal {\\n    ...moneyFields\\n  }\\n  updatedFeeTotal {\\n    ...moneyFields\\n  }\\n  updatedShippingTotal {\\n    ...moneyFields\\n  }\\n  updatedTaxTotal {\\n    ...moneyFields\\n  }\\n  parentOperationNumber\\n  parentOperationId\\n  vendor {\\n    name\\n    id\\n  }\\n  approvalRequest {\\n    ...opusOrderApprovalRequestFields\\n  }\\n  dynamicProperties {\\n    name\\n    value\\n  }\\n  supplierOrders {\\n    id\\n    number\\n    comment\\n    createdDate\\n    status\\n    purchaseOrderNumber\\n    discounts {\\n      amount {\\n        amount\\n        formattedAmount\\n        currency {\\n          code\\n        }\\n      }\\n      coupon\\n      description\\n      promotionId\\n    }\\n    taxTotal {\\n      ...moneyFields\\n    }\\n    subTotal {\\n      ...moneyFields\\n    }\\n    total {\\n      ...moneyFields\\n    }\\n    discountTotal {\\n      ...moneyFields\\n    }\\n    shippingTotal {\\n      ...moneyFields\\n    }\\n    currency {\\n      ...currencyFields\\n    }\\n    items {\\n      ...opusOrderLineItemFields\\n    }\\n    shipments {\\n      id\\n      shipmentMethodCode\\n      shipmentMethodOption\\n      shippingMethod {\\n        logoUrl\\n        typeName\\n      }\\n      price {\\n        ...moneyFields\\n      }\\n      deliveryAddress {\\n        ...orderAddressFields\\n      }\\n      comment\\n    }\\n    inPayments(sort: \\\&quot;CreatedDate:desc\\\&quot;) {\\n      id\\n      number\\n      isApproved\\n      gatewayCode\\n      status\\n      comment\\n      paymentMethod {\\n        logoUrl\\n        code\\n        typeName\\n        paymentMethodType\\n        paymentMethodGroupType\\n      }\\n      billingAddress {\\n        ...orderAddressFields\\n      }\\n      dynamicProperties {\\n        name\\n        value\\n      }\\n    }\\n    availablePaymentMethods {\\n      code\\n      logoUrl\\n      price {\\n        ...moneyFields\\n      }\\n      settings {\\n        key\\n        value\\n      }\\n    }\\n    vendor {\\n      name\\n      id\\n    }\\n    dynamicProperties {\\n      name\\n      value\\n    }\\n  }\\n}\\n\\nfragment moneyFields on MoneyType {\\n  amount\\n  formattedAmount\\n  formattedAmountWithoutCurrency\\n  currency {\\n    ...currencyFields\\n  }\\n}\\n\\nfragment currencyFields on CurrencyType {\\n  code\\n  symbol\\n}\\n\\nfragment shortOrderFields on OpusCustomerOrderType {\\n  id\\n  number\\n  createdDate\\n  status\\n  items {\\n    ...opusOrderLineItemFields\\n  }\\n  discounts {\\n    amount {\\n      amount\\n      formattedAmount\\n      currency {\\n        code\\n      }\\n    }\\n    coupon\\n    description\\n    promotionId\\n  }\\n  discountTotal {\\n    ...moneyFields\\n  }\\n  shippingTotal {\\n    ...moneyFields\\n  }\\n  taxTotal {\\n    ...moneyFields\\n  }\\n  subTotal {\\n    ...moneyFields\\n  }\\n  total {\\n    ...moneyFields\\n  }\\n}\\n\\nfragment opusOrderLineItemFields on OpusOrderLineItemType {\\n  id\\n  imageUrl\\n  isGift\\n  name\\n  productId\\n  productType\\n  product {\\n    id\\n    brandName\\n    slug\\n    masterVariation {\\n      id\\n      slug\\n    }\\n    properties {\\n      ...propertyFields\\n    }\\n    keyProperties {\\n      ...propertyFields\\n    }\\n  }\\n  quantity\\n  sku\\n  extendedPrice {\\n    ...moneyFields\\n  }\\n  placedPrice {\\n    ...moneyFields\\n  }\\n  taxTotal {\\n    ...moneyFields\\n  }\\n  discountTotal {\\n    ...moneyFields\\n  }\\n  outerId\\n  vendor {\\n    id\\n    name\\n    rating {\\n      value\\n      reviewCount\\n    }\\n  }\\n}\\n\\nfragment propertyFields on Property {\\n  name\\n  value\\n  type\\n  hidden\\n  valueType\\n  label\\n}\\n\\nfragment orderAddressFields on OrderAddressType {\\n  id\\n  name\\n  organization\\n  firstName\\n  lastName\\n  line1\\n  line2\\n  city\\n  countryCode\\n  countryName\\n  regionId\\n  regionName\\n  postalCode\\n  phone\\n  email\\n  addressType\\n}\\n\\nfragment opusOrderApprovalRequestFields on OpusOrderApprovalRequestType {\\n  id\\n  mainOrderId\\n  mainOrderNumber\\n  approverId\\n  customerId\\n  approverName\\n  approverEmail\\n  customerName\\n  customerEmail\\n  createdBy\\n  createdDate\\n  modifiedBy\\n  modifiedDate\\n  status\\n  orderStatus\\n  orderTotal\\n  comment\\n  resolvedByName\\n  resolvedById\\n}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c0141233-e5b2-4c87-ad29-33a320387433</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.token}</value>
      <webElementGuid>776c4bc5-779d-4f0d-beb8-c96a5e92805e</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'5a4994c0-5c95-4985-bf99-9ec3e6a5239a'</defaultValue>
      <description></description>
      <id>74163946-d180-451b-98e8-d93205c22596</id>
      <masked>false</masked>
      <name>cartId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonOutput
import com.kms.katalon.core.util.KeywordUtil


ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
def prettyJson = JsonOutput.prettyPrint(response.getResponseBodyContent())
KeywordUtil.logInfo(prettyJson)
WS.verifyResponseStatusCode(response, 200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
