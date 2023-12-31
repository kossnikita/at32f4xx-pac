#[doc = "Register `DIEPTXF0` reader"]
pub type R = crate::R<DIEPTXF0_SPEC>;
#[doc = "Register `DIEPTXF0` writer"]
pub type W = crate::W<DIEPTXF0_SPEC>;
#[doc = "Field `INEPT0TXSTADDR` reader - Endpoint 0 transmit RAM start address"]
pub type INEPT0TXSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEPT0TXSTADDR` writer - Endpoint 0 transmit RAM start address"]
pub type INEPT0TXSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPT0TXDEP` reader - Endpoint 0 TxFIFO depth"]
pub type INEPT0TXDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEPT0TXDEP` writer - Endpoint 0 TxFIFO depth"]
pub type INEPT0TXDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn inept0txstaddr(&self) -> INEPT0TXSTADDR_R {
        INEPT0TXSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn inept0txdep(&self) -> INEPT0TXDEP_R {
        INEPT0TXDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF0")
            .field(
                "inept0txstaddr",
                &format_args!("{}", self.inept0txstaddr().bits()),
            )
            .field(
                "inept0txdep",
                &format_args!("{}", self.inept0txdep().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIEPTXF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn inept0txstaddr(&mut self) -> INEPT0TXSTADDR_W<DIEPTXF0_SPEC> {
        INEPT0TXSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn inept0txdep(&mut self) -> INEPT0TXDEP_W<DIEPTXF0_SPEC> {
        INEPT0TXDEP_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IN Endpoint TxFIFO 0 transmit FIFO size register (Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF0_SPEC;
impl crate::RegisterSpec for DIEPTXF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf0::R`](R) reader structure"]
impl crate::Readable for DIEPTXF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf0::W`](W) writer structure"]
impl crate::Writable for DIEPTXF0_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTXF0 to value 0x0200"]
impl crate::Resettable for DIEPTXF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
