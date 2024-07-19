import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import groovy.json.JsonSlurper
import internal.GlobalVariable

WebUI.comment('TEST CASE: setup supplier portal backend')


//WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleSearch/BuildIndex'),null)

'CREATE KATALON OPERATOR'
GlobalVariable.urlBack = GlobalVariable.supplierPortalUrl
WS.callTestCase(findTestCase('Test Cases/KatalonSampleDataOperations/KatalonOperatorCreate'), null)


'CREATE MASTER CATALOG'//hardcoded put request
masterCatalogCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.Catalog/masterCatalogCreate'))


'CREATE MAIN CATEGORY'
GlobalVariable.categoryName = 'main category'
categoryCreate = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Catalog/CategoriesCreate', [
	('catalogId') : GlobalVariable.supplierPortalMasterCatalogId,
	('code') : 'MFD',
	('name') : 'main category'
	]))
//WS.verifyElementPropertyValue(categoryCreate, 'catalogId', GlobalVariable.catalogId)
//WS.verifyElementPropertyValue(categoryCreate, 'name', GlobalVariable.categoryName)
//WS.verifyElementPropertyValue(categoryCreate, 'code', GlobalVariable.categoryCode)


'CREATE CURRENCY'
currencyCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Core/currencyCreate',[
	('currencyName') : "USD",
	('currencyCode') : "USD"
	]))


'CREATE MASTER STORE (MFD) WITH ALL THE REQUIRED SETTINGS'//hardcoded to avoid long and cumbersome process of create update
masterStoreCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.Store/masterStoreCreateVSP'))


'GET ORDER MODULE SETTINGS'
moduleId = 'VirtoCommerce.Orders'
settingsGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/SettingsGetByModuleId',[
	('orderId') : moduleId
	]))


'GET ORDER STATUSES OBJECT'
orderStatusResponse = settingsGet.getResponseBodyContent()


'TO THE OBJECT ADD A CONFIRMED STATUS'
newStatus = 'Confirmed'
orderStatusesParsed = new JsonSlurper().parseText(orderStatusResponse)
orderStatusesObject = orderStatusesParsed.find { it.name == "Order.Status" }
orderStatusesObject.allowedValues << newStatus
orderStatusesObject.defaultValue = "New"
updatedOrderStatuses = '[' + new groovy.json.JsonBuilder(orderStatusesObject).toString() + ']'
println updatedOrderStatuses


'SEND REQUEST TO UPDATE ORDER STATUSES'
RequestObject updatedOrderStatusesRequest = findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/SettingsUpdateBoolean')
updatedOrderStatusesRequest.setBodyContent(new HttpTextBodyContent(updatedOrderStatuses))
statusesUpdate = WS.sendRequestAndVerify(updatedOrderStatusesRequest)


'VERIFY STATUS HAS BEEN UPDATED'
updatedSettingsGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/SettingsGetByModuleId',[
	('orderId') : moduleId
	]))
WS.containsString(updatedSettingsGet, newStatus, false)

