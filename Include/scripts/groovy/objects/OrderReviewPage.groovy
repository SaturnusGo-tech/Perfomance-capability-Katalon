package objects

import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import org.junit.Assert


import org.openqa.selenium.WebDriver
import org.openqa.selenium.JavascriptExecutor
import com.kms.katalon.core.webui.driver.DriverFactory
import org.openqa.selenium.support.ui.WebDriverWait

import java.net.HttpURLConnection
import java.net.URL


/**
 * The Review class encapsulates actions and behaviors specific to the review page.
 */
class Review {
	static final String SEND_TO_APPROVAL = "SEND_TO_APPROVAL"

	/**
	 * Clicks the 'Send to Approval' button on the review page.
	 */
	static void clickSendToApprovalButton() {
		// Wait for the 'Send to Approval' button to be present
		boolean isButtonPresent = WebUI.waitForElementPresent(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/Order_VerifyStage/' + SEND_TO_APPROVAL), 10)

		// Assert that the 'Send to Approval' button is present
		Assert.assertTrue("The 'Send to Approval' button is not present.", isButtonPresent)

		// Click the 'Send to Approval' button
		WebUI.click(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/Order_VerifyStage/' + SEND_TO_APPROVAL))
	}
	/**
	 * Checks the page load performance by verifying the page URL and HTTP status.
	 * @param pageUrl The URL of the page to be checked.
	 * @param timeout The timeout for page load in milliseconds.
	 */
	static void checkPageLoadPerformance(String pageUrl, int timeout) {
		// Check the HTTP status of the page
		HttpURLConnection connection = (HttpURLConnection) (new URL(pageUrl).openConnection())
		connection.setRequestMethod("GET")
		connection.connect()
		int responseCode = connection.getResponseCode()
		assert responseCode == 200 : "Expected HTTP status 200, but received " + responseCode

		// Measure the page load time
		WebDriver driver = DriverFactory.getWebDriver()
		long startTime = System.currentTimeMillis()
		driver.get(pageUrl)

		// Convert timeout from milliseconds to seconds for WebDriverWait
		int timeoutInSeconds = timeout / 1000
		new WebDriverWait(driver, timeoutInSeconds).until({ WebDriver d ->
			return ((JavascriptExecutor) d).executeScript("return document.readyState").equals("complete")
		} as com.google.common.base.Function<WebDriver, Boolean>)

		long endTime = System.currentTimeMillis()

		// Page load time in milliseconds
		long loadTime = endTime - startTime
		println("Page load time: " + loadTime + " ms")

		// Check if the page load time exceeds the specified timeout
		assert loadTime <= timeout : "Page load time exceeds " + timeout + " ms"
	}
}


