import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import groovy.json.JsonSlurper
import internal.GlobalVariable


WebUI.comment('TEST CASE: complete order')


'SWITCH DEFAULT URL TO VSP'
initialUrl = GlobalVariable.urlBack
GlobalVariable.urlBack = GlobalVariable.supplierPortalUrl


'GET THE CREATED VSP ORDER BY ID'
vspOrderGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Order/OrderGetById',[
	('orderId') : GlobalVariable.supplierOrderId
	]))
vspOrderResponse = vspOrderGet.getResponseBodyContent()


'PREPARE VSP ORDER STATUS UPDATE REQUEST BODY'
vspOrderParsed = new JsonSlurper().parseText(vspOrderResponse)
expectedOrderStatus = 'Completed'
vspOrderParsed.status = expectedOrderStatus 
vspOrderUpdatedJson = new groovy.json.JsonBuilder(vspOrderParsed).toString()
println vspOrderUpdatedJson


'SEND REQUEST TO UPDATE VSP ORDER STATUS'
RequestObject vspOrderUpdateRequest = findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Order/OrderUpdate')
vspOrderUpdateRequest.setBodyContent(new HttpTextBodyContent(vspOrderUpdatedJson))
vspOrderUpdate = WS.sendRequestAndVerify(vspOrderUpdateRequest)


'VERIFY VSP ORDER STATUS HAS BEEN UPDATED'
vspUpdatedOrderGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Order/OrderGetById',[
	('orderId') : GlobalVariable.supplierOrderId
	]))
WS.verifyElementPropertyValue(vspUpdatedOrderGet,'status', expectedOrderStatus)


'SWITCH BACK TO DEFAULT URL VALUE'
GlobalVariable.urlBack = initialUrl
println GlobalVariable.urlBack


'EVALUATE ORDER ON CUSTOMER BACKEND'
backendOrderEvaluate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/orderReevaluate',[
	('orderId') : GlobalVariable.orderId
	]))


'VERIFY ORDERS STATUSES ARE UPDATED ON CUSTOMER BACKEND'
ordersIdList = [GlobalVariable.orderId, GlobalVariable.supplierOrderId]
for (int i; i < ordersIdList.size();i++) {
	orderGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Order/OrderGetById',[
		('orderId') : ordersIdList[i]
		]))
	WS.verifyElementPropertyValue(orderGet,'status',expectedOrderStatus)
}




