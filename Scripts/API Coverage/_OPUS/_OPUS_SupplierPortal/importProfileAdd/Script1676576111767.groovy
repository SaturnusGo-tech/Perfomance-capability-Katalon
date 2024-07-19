import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

WebUI.comment('TEST CASE: CREATE IMPORT PROFILE AND IMPORT PRODUCTS')


'GET ADMIN TOKEN TO AUTHORIZE THE REQUEST'
adminTokenGet = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModulePlatform/Authorization/AuthBackToken'),null)


'CREATE IMPORT PROFILE'
profileCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.marketplaceVendorExtension/importProfileAdd',[
	('name') : GlobalVariable.katalonImportProfileName ,
	('sellerId') : GlobalVariable.katalonVendorId,
	('username') : GlobalVariable.katalonVendorName,
	('dataImporterType') : GlobalVariable.dataImporterType
	]))


'VERIFY THE PROFILE HAS BEEN CREATED AND GET ITS ID'
profileVerifyCreated = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.marketplaceVendorExtension/importProfileSearch',[
	('keyword') : GlobalVariable.katalonImportProfileName 
	]))
WS.verifyElementPropertyValue(profileVerifyCreated,'results[0].name' , GlobalVariable.katalonImportProfileName )
GlobalVariable.katalonImportProfileId = WS.getElementPropertyValue(profileVerifyCreated, 'results[0].id')

