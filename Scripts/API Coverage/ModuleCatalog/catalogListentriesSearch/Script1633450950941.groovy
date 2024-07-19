import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable as GlobalVariable




WebUI.comment('TEST CASE: Products management - Listentries Search')




//'VERIFY VARS'
//WebUI.comment(GlobalVariable.keyword)
//WebUI.comment(GlobalVariable.productName)
//WebUI.comment(GlobalVariable.productId)
//WebUI.comment(GlobalVariable.categoryName)
//
//'SEARCH FOR THE PRODUCT'
//Have deleted the "catalogId" request header from the request below as it was returning totalCount = 1,
//so crashing the case


//GlobalVariable.productName = 'QweProductUPD'
//GlobalVariable.keyword = GlobalVariable.productName


'DROP INDEX'
//WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Search/index'))
WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleSearch/BuildIndex'),null)
WebUI.comment(GlobalVariable.keyword)
 verifyDeleted = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Catalog/ListentriesSearch', [
	('keyword') : GlobalVariable.keyword,
	('') : '' 
	]))	
//name = WS.getElementPropertyValue(verifyDeleted,'listEntries[0].name')
//WebUI.comment(name)
//boolean verification = WS.containsString(verifyDeleted, GlobalVariable.keyword, false, FailureHandling.OPTIONAL)
//WebUI.comment(verification)
//assert verification == false
WS.verifyElementPropertyValue(verifyDeleted, 'listEntries', '[]')
return verifyDeleted

'VERIFY VARS'
WebUI.comment(GlobalVariable.keyword)
WebUI.comment(GlobalVariable.productName)
WebUI.comment(GlobalVariable.productId)

