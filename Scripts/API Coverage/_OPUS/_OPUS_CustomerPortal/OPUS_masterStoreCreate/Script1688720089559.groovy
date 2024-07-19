import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import groovy.json.JsonSlurper
import internal.GlobalVariable

WebUI.comment('TEST CASE: create OPUS store')


'CREATE OPUS STORE'
WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.Store/masterStoreCreateBackend'))


'SEARCH FOR PAYMENT AND GET ITS DATA TO UPDATE THE STORE'
storeId = 'opus'
paymentSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Payment/paymentSearch',[
	('storeId') : storeId,
	('keyword') : GlobalVariable.paymentMethod
	]))
responseBody = paymentSearch.getResponseBodyContent()
responseBodyParsed = new JsonSlurper().parseText(responseBody)
paymentMethodBody = responseBodyParsed.results[0]
paymentMethodBody.isActive = 'true'
paymentMethodBody.storeId = storeId
paymentJson = new groovy.json.JsonBuilder(paymentMethodBody).toString()


'SEND REQUEST TO UPDATE STORE PAYMENT METHOD'
RequestObject paymentUpdateRequest = findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Payment/paymentUpdate')
paymentUpdateRequest.setBodyContent(new HttpTextBodyContent(paymentJson))
paymentUpdate = WS.sendRequestAndVerify(paymentUpdateRequest)



'SEARCH FOR SHIPPING AND GET ITS DATA TO UPDATE THE STORE'
//to switch between FixedRate and SupplierDefaultShippingMethod
//set the related global variable and comment/uncomment lines 32,33x
shippingName = GlobalVariable.shippingMethod
storeId = 'opus'
getShippingMethod = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Shipping/shippingSearch',[
	('storeId') : storeId//,
	//('keyword') : shippingName
	]))
//there's an issue via search by keyword (returns 500. Commented untill investigated.
result = getShippingMethod.getResponseBodyContent()
resultParsed = new JsonSlurper().parseText(result)
shipping = resultParsed.results[0]
shipping.isActive = 'true'
shipping.storeId = storeId
shipping.settings[0].value = GlobalVariable.shippingRateValue
shipping.settings[1].value = GlobalVariable.shippingRateValue



shippingJson = new groovy.json.JsonBuilder(shipping).toString()
println shippingJson


'SEND REQUEST TO UPDATE STORE DEFAULT SHIPPING METHOD'
RequestObject shipmentUpdateRequest = findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Shipping/shippingUpdate')
shipmentUpdateRequest.setBodyContent(new HttpTextBodyContent(shippingJson))
shipmentUpdate = WS.sendRequestAndVerify(shipmentUpdateRequest)

