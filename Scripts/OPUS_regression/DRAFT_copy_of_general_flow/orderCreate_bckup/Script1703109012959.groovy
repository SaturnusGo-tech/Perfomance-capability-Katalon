import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.UrlEncodedBodyParameter
import com.kms.katalon.core.testobject.impl.HttpUrlEncodedBodyContent
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import groovy.json.JsonSlurper
import internal.GlobalVariable


// productId storeId
WebUI.comment('TEST CASE: Create order')
//Do not forget to set actual supplierId in local profile if debugging and launching locally


'GET AGENCY USER TOKEN'
def request = findTestObject('API/backWebServices/VirtoCommerce.Platform/Authorization/AuthorizationToken')
List<UrlEncodedBodyParameter> body = new ArrayList()
body.add(new UrlEncodedBodyParameter('grant_type', 'password'))
body.add(new UrlEncodedBodyParameter('scope', 'offline_access'))
body.add(new UrlEncodedBodyParameter('username', GlobalVariable.agencyUserName))
body.add(new UrlEncodedBodyParameter('password', GlobalVariable.katalonVendorPassword))

request.setBodyContent(new HttpUrlEncodedBodyContent(body))
response = WS.sendRequestAndVerify(request)


// STEP | Parse request and save token to the GlobalVariable
def responseJson = new JsonSlurper().parseText(response.getResponseBodyContent())
GlobalVariable.token = ((responseJson.token_type + ' ') + responseJson.access_token)
WebUI.comment(GlobalVariable.token)


'GET USERID'
userGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_userGet'))
userName = WS.getElementPropertyValue(userGet,'data.me.userName')
userId = WS.getElementPropertyValue(userGet,'data.me.id')
String regex = "\\[|\\]";
userId = userId.toString()
userId = userId.replaceAll(regex, "");
println userId


'GET A RANDOM PRODUCT'
storeId = 'opus'
println new Random().nextInt(100)
productsList = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_productsSearch',[
	('storeId') : storeId,
	('userId') : userId
	]))
productId = WS.getElementPropertyValue(productsList, 'data.products.items[' + new Random().nextInt(100) + '].id')
println productId
 
 
'GET A PRODUCT PRICE FOR FURTHER ORDER VERIFICATION'
priceGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_offerGet',[
	('productId') : productId
	]))
productPrice = WS.getElementPropertyValue(priceGet, 'data.product.price.actual.amount')
println productPrice


'CART ADD ITEM'
cartAddItem = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_cartAddItem',[
	('storeId'): storeId,
	('userId'): userId,
	('productId'): productId,
	]))


'GET FULL CART DATA'
storeId = 'opus'
cartGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_cartGetFull',[
	('storeId') : storeId,
	('userId') : userId
	]))
GlobalVariable.cartId = WS.getElementPropertyValue(cartGet,'data.cart.id')
println userId
println userName
println GlobalVariable.cartId


'GET AND FORMAT ADDRESSES INFORMATION'
addressesGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_addressesGet'))
addressesGetBody = addressesGet.getResponseBodyContent()
adressesGetBodyParsed = new JsonSlurper().parseText(addressesGetBody)
//prepare billing address json
billingAddressParsed = adressesGetBodyParsed.data.me.contact.billingAddresses.items[0]
billingAddressParsed.remove('isDefault')
billingAddressParsed.remove('description')
billingAddressJson = new groovy.json.JsonBuilder(billingAddressParsed).toString()
//prepare shipping address json
shippingAddressParsed = adressesGetBodyParsed.data.me.contact.shippingAddresses.items[0]
shippingAddressParsed.remove('isDefault')
shippingAddressParsed.remove('description')
shippingAddressJson = new groovy.json.JsonBuilder(shippingAddressParsed).toString()
println shippingAddressJson
println userId


'ADD SHIPMENT ADDRESS'
cartShipmentAddressAdd = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_cartAddShipmentAddress',[
	('storeId') : storeId,
	('userId') : userId,
	('cartId') : GlobalVariable.cartId,
	('address') : shippingAddressJson,
	('vendorId') : GlobalVariable.supplierId
	]))
shipmentId = WS.getElementPropertyValue(cartShipmentAddressAdd,'data.addOrUpdateCartShipment.shipments[0].id')
validationErrors = WS.getElementPropertyValue(cartShipmentAddressAdd,'data.addOrUpdateCartShipment.validationErrors')
assert validationErrors == null || validationErrors == []


'COLLECT SHIPMENT DATA'
shipmentMethodId = WS.getElementPropertyValue(cartShipmentAddressAdd,'data.addOrUpdateCartShipment.availableShippingMethods[0].id')
shipmentMethodCode = WS.getElementPropertyValue(cartShipmentAddressAdd,'data.addOrUpdateCartShipment.availableShippingMethods[0].code')
shipmentMethodOption = WS.getElementPropertyValue(cartShipmentAddressAdd,'data.addOrUpdateCartShipment.availableShippingMethods[0].optionName')
shipmentMethodAmount = WS.getElementPropertyValue(cartShipmentAddressAdd,'data.addOrUpdateCartShipment.availableShippingMethods[0].price.amount')
println shipmentMethodId
println shipmentMethodCode
println shipmentMethodOption
println shipmentMethodAmount

'ADD SHIPPING METHOD'
//shipmentName = GlobalVariable.shippingMethod
shippingMethodAdd = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_cartAddShipmentMethod',[
	('storeId') : storeId,
	('userId') : userId,
	('cartId') : GlobalVariable.cartId,
	('vendorId') : GlobalVariable.supplierId,
	('shipmentMethodCode') : shipmentMethodCode,
	('shipmentMethodAmount') : shipmentMethodAmount,
	('shipmentMethodOption') : shipmentMethodOption,
	('shipmentId') : shipmentId
	]))
validationErrors = WS.getElementPropertyValue(shippingMethodAdd,'data.addOrUpdateCartShipment.validationErrors')
assert validationErrors == null || validationErrors == []


'GET FULL CART DATA'
storeId = 'opus'
cartGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_cartGetFull',[
	('storeId') : storeId,
	('userId') : userId
	]))

'COLLECT PAYMENT DATA (from the updated cart)'
cartGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_cartGetFull',[
	('storeId') : storeId,
	('userId') : userId
	]))
paymentMethodCode = WS.getElementPropertyValue(cartGet,'data.cart.availablePaymentMethods[0].code')
paymentMethodName = WS.getElementPropertyValue(cartGet,'data.cart.availablePaymentMethods[0].name')
paymentMethodgroupType = WS.getElementPropertyValue(cartGet,'data.cart.availablePaymentMethods[0].paymentMethodGroupType')
//paymentMethodAmount = WS.getElementPropertyValue(cartGet,'data.cart.payments[0].amount.amount')


'CREATE BILLING'
cartbillingAddressAdd = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_cartAddPaymentCreate',[
	('storeId') : storeId,
	('userId') : userId,
	('cartId') : GlobalVariable.cartId,
	('address') : billingAddressJson,
	('vendorId') : GlobalVariable.supplierId,
	('paymentGatewayCode') : paymentMethodCode
	]))
validationErrors = WS.getElementPropertyValue(cartbillingAddressAdd,'data.addOrUpdateCartPayment.validationErrors')
assert validationErrors == null || validationErrors == []
paymentId = WS.getElementPropertyValue(cartbillingAddressAdd,'data.addOrUpdateCartPayment.payments[0].id')
paymentMethodAmount = WS.getElementPropertyValue(cartbillingAddressAdd,'data.addOrUpdateCartPayment.payments[0].amount.amount')



'ADD BILLING METHOD'
billingAdd = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_cartAddPaymentUpdate',[
	('storeId') : storeId,
	('userId') : userId,
	('cartId') : GlobalVariable.cartId,
	('vendorId') : GlobalVariable.supplierId,
	('paymentId') : paymentId,
	('billingAddress') : billingAddressJson,
	('amount') : paymentMethodAmount
	]))
validationErrors = WS.getElementPropertyValue(billingAdd,'data.addOrUpdateCartPayment.validationErrors')
assert validationErrors == null || validationErrors == []


'GET FULL CART DATA'
storeId = 'opus'
cartGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_cartGetFull',[
	('storeId') : storeId,
	('userId') : userId
	]))


'CREATE ORDER FROM CART'
orderCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_orderCreateFromCart',[
	('cartId') : GlobalVariable.cartId
	]))


'GET CREATED ORDER DATA'
orderNumber = WS.getElementPropertyValue(orderCreate,'data.createOrderFromCart.number')
supplierOrderNumber = WS.getElementPropertyValue(orderCreate,'data.createOrderFromCart.supplierOrders[0].number')
//regex is set at line 40
GlobalVariable.orderNumber = orderNumber.toString().replaceAll(regex, "")
GlobalVariable.supplierOrderNumber = supplierOrderNumber.toString().replaceAll(regex, "")


'REMOVE CART'
cartRemove = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/GRAPHQL/GQL_cartRemove',[
	('userId') : userId,
	('cartId') : GlobalVariable.cartId
	]))
WS.verifyElementPropertyValue(cartRemove, 'data.removeCart', true)


'VERIFY ORDERS ARE ACCESSIBLE ON THE BACKEND AND HAVE RELEVANT DATA'
//convert shippingPrice to match the format
ordersList = [GlobalVariable.orderNumber, GlobalVariable.supplierOrderNumber]
for(int i; i < ordersList.size(); i++) {
	backendOrderSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Order/OrderSearch',[
		('keyword') : ordersList[i]
		]))
	WS.verifyElementPropertyValue(backendOrderSearch,'totalCount', 1)
	WS.verifyElementPropertyValue(backendOrderSearch,'results[0].number', ordersList[i])
	println ordersList[i]
	//verify order total calculation
	orderTaxTotal = WS.getElementPropertyValue(backendOrderSearch,'results[0].taxTotal')
	orderTotal = WS.getElementPropertyValue(backendOrderSearch,'results[0].total')
	assert orderTotal == ( orderTaxTotal + productPrice + shipmentMethodAmount )
	//verify order has a relevant item
	orderId = WS.getElementPropertyValue(backendOrderSearch,'results[0].id')
	orderGetById = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Order/OrderGetById',[
		('orderId') : orderId
		]))
	orderItemId = WS.getElementPropertyValue(orderGetById,'items[0].productId')
	assert orderItemId == productId
	//end
	}

	