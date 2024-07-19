import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable


WebUI.comment('TEST CASE: Order evaluate')


'GET CUSTOMER ORDER ID'
backendOrderSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Order/OrderSearch',[
	('keyword') : GlobalVariable.orderNumber
	]))
GlobalVariable.orderId = WS.getElementPropertyValue(backendOrderSearch,'results[0].id')


'GET SUPPLIER ORDER ID'
backendSupplierOrderSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Order/OrderSearch',[
	('keyword') : GlobalVariable.supplierOrderNumber
	]))
GlobalVariable.supplierOrderId = WS.getElementPropertyValue(backendSupplierOrderSearch,'results[0].id')
initialSupplierOrderStatus = WS.getElementPropertyValue(backendSupplierOrderSearch,'results[0].status')



'EVALUATE ORDER UNLESS ACTUAL STATUS IS UPDATED AND VERIFY RESULTS'
actualPaymentStatus = null
actualSupplierOrderStatus = initialSupplierOrderStatus
while (initialSupplierOrderStatus == actualSupplierOrderStatus ) {
	
	'SEND REQUEST TO EVALUATE ORDER'
	orderEvaluate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/orderReevaluate',[
		('orderId') : GlobalVariable.orderId 
		]))
	
	'VERIFY SUPPLIER ORDER STATUS & PAYMENT STATUS HAVE BEEN UPDATED'
	supplierOrderGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Order/OrderGetById',[
		('orderId') : GlobalVariable.supplierOrderId
		]))
	actualSupplierOrderStatus = WS.getElementPropertyValue(supplierOrderGet,'status')
	actualPaymentStatus = WS.getElementPropertyValue(supplierOrderGet,'inPayments[0].paymentStatus')
	}
	expectedOrderStatus = 'Pending'
	expectedPaymentStatus = 'Paid'
	assert actualSupplierOrderStatus == expectedOrderStatus
	assert actualPaymentStatus == expectedPaymentStatus


'VERIFY ORDER PAYMENT STATUS HAS BEEN UPDATED'
orderGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Order/OrderGetById',[
	('orderId') : GlobalVariable.orderId
	]))
paymentStatus = WS.getElementPropertyValue(orderGet,'inPayments[0].paymentStatus')
assert paymentStatus == expectedPaymentStatus


'SWITCH DEFAULT URL TO VSP'
initialUrl = GlobalVariable.urlBack
GlobalVariable.urlBack = GlobalVariable.supplierPortalUrl


'VERIFY ORDER HAS BEEN SENT TO VSP'
vspSupplierOrderGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Order/OrderGetById',[
	('orderId') : GlobalVariable.supplierOrderId
	]))
vspOrderNumber = WS.getElementPropertyValue(vspSupplierOrderGet,'number')
assert vspOrderNumber == GlobalVariable.supplierOrderNumber


'SWITCH BACK TO DEFAULT URL VALUE'
GlobalVariable.urlBack = initialUrl
println GlobalVariable.urlBack



