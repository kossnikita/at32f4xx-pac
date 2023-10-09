#[doc = "Register `DIEPEMPMSK` reader"]
pub type R = crate::R<DIEPEMPMSK_SPEC>;
#[doc = "Register `DIEPEMPMSK` writer"]
pub type W = crate::W<DIEPEMPMSK_SPEC>;
#[doc = "Field `INEPTXFEMSK` reader - IN EP Tx FIFO empty interrupt mask bits"]
pub type INEPTXFEMSK_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFEMSK` writer - IN EP Tx FIFO empty interrupt mask bits"]
pub type INEPTXFEMSK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits"]
    #[inline(always)]
    pub fn ineptxfemsk(&self) -> INEPTXFEMSK_R {
        INEPTXFEMSK_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPEMPMSK")
            .field(
                "ineptxfemsk",
                &format_args!("{}", self.ineptxfemsk().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIEPEMPMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfemsk(&mut self) -> INEPTXFEMSK_W<DIEPEMPMSK_SPEC, 0> {
        INEPTXFEMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGHS device IN endpoint FIFO empty interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPEMPMSK_SPEC;
impl crate::RegisterSpec for DIEPEMPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepempmsk::R`](R) reader structure"]
impl crate::Readable for DIEPEMPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepempmsk::W`](W) writer structure"]
impl crate::Writable for DIEPEMPMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPEMPMSK to value 0"]
impl crate::Resettable for DIEPEMPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
