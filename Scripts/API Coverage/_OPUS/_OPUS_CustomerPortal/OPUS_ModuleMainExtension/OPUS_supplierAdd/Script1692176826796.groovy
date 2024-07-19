import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable


WebUI.comment('TEST CASE: ADD SUPPLIER')


customerPortalUrl = GlobalVariable.urlBack
supplierPortalUrl = GlobalVariable.supplierPortalUrl


'SWITCH GLOBAL URL TO OPERATE ON SUPPLIER PORTAL'
GlobalVariable.urlBack = supplierPortalUrl


'SEARCH FOR THE VENDOR TO VERIFY IT WAS CREATED AND GET ID'
vendorSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.MarketplaceVendorExtension/sellerSearch',[
	('keyword') : GlobalVariable.katalonVendorName
	]))
WS.verifyElementPropertyValue(vendorSearch, 'results[0].name', GlobalVariable.katalonVendorName)
GlobalVariable.katalonVendorId = WS.getElementPropertyValue(vendorSearch, 'results[0].id')
GlobalVariable.supplierOuterId = GlobalVariable.katalonVendorRegistrationId


'SWITCH GLOBAL URL BACK TO CUSTOMER PORTAL'
GlobalVariable.urlBack = customerPortalUrl


'ADD SUPPLIER'
supplierAdd = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/suppliersAdd',[
	('name') : GlobalVariable.katalonVendorName,
	('outerId') : GlobalVariable.supplierOuterId
	]))


'LAUNCH INDEXATION'
indexRun = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleSearch/BuildIndex'),null)


'VERIFY THE SUPPLER HAS BEEN CREATED AND GET THE SUPPLIER ID'
supplierSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/suppliersSearch',[
	('keyword') : GlobalVariable.katalonVendorName
	]))
WS.verifyElementPropertyValue(supplierSearch, 'results[0].name', GlobalVariable.katalonVendorName)
GlobalVariable.supplierId = WS.getElementPropertyValue(supplierSearch,'results[0].id')

