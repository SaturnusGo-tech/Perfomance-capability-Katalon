import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

WebUI.comment('TEST CASE: Get OPUS catalog data')


'GET THE LIST OF ALL PRODUCT AND EXTRACT THE NAME OF A FIRST IN THE LIST'
customerPortalProductData = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Catalog/ProductsSearch',[
	('keyword') : ''
	]))
GlobalVariable.OPUS_productName = WS.getElementPropertyValue(customerPortalProductData,'items[0].name')


'OPUS CATALOG DATA GET'
opusProductsGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Catalog/ProductsSearch',[
	('keyword') : GlobalVariable.OPUS_productName
	]))


'VERIFY THE EXPECTED PRODUCT HAS BEEN PICKED AND COLLECT DATA'
assert GlobalVariable.OPUS_productName == WS.getElementPropertyValue(opusProductsGet, 'items[0].name')
GlobalVariable.OPUS_productId = WS.getElementPropertyValue(opusProductsGet, 'items[0].id')
GlobalVariable.OPUS_productSKU = WS.getElementPropertyValue(opusProductsGet, 'items[0].code')

println GlobalVariable.OPUS_productName 
println GlobalVariable.OPUS_productId  
println GlobalVariable.OPUS_productSKU  
