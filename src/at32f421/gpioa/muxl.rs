#[doc = "Register `MUXL` reader"]
pub type R = crate::R<MUXL_SPEC>;
#[doc = "Register `MUXL` writer"]
pub type W = crate::W<MUXL_SPEC>;
#[doc = "Field `MUX[0-7]` reader - GPIOx pin %s muxing"]
pub type MUX_R = crate::FieldReader;
#[doc = "Field `MUX[0-7]` writer - GPIOx pin %s muxing"]
pub type MUX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "GPIOx pin [0-7]
muxing\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn mux(&self, n: u8) -> MUX_R {
        assert!(n < 8);
        MUX_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - GPIOx pin 0 muxing"]
    #[inline(always)]
    pub fn mux0(&self) -> MUX_R {
        MUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - GPIOx pin 1 muxing"]
    #[inline(always)]
    pub fn mux1(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - GPIOx pin 2 muxing"]
    #[inline(always)]
    pub fn mux2(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - GPIOx pin 3 muxing"]
    #[inline(always)]
    pub fn mux3(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - GPIOx pin 4 muxing"]
    #[inline(always)]
    pub fn mux4(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - GPIOx pin 5 muxing"]
    #[inline(always)]
    pub fn mux5(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - GPIOx pin 6 muxing"]
    #[inline(always)]
    pub fn mux6(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - GPIOx pin 7 muxing"]
    #[inline(always)]
    pub fn mux7(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXL")
            .field("mux0", &format_args!("{}", self.mux0().bits()))
            .field("mux1", &format_args!("{}", self.mux1().bits()))
            .field("mux2", &format_args!("{}", self.mux2().bits()))
            .field("mux3", &format_args!("{}", self.mux3().bits()))
            .field("mux4", &format_args!("{}", self.mux4().bits()))
            .field("mux5", &format_args!("{}", self.mux5().bits()))
            .field("mux6", &format_args!("{}", self.mux6().bits()))
            .field("mux7", &format_args!("{}", self.mux7().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MUXL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "GPIOx pin [0-7]
muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self, n: u8) -> MUX_W<MUXL_SPEC> {
        assert!(n < 8);
        MUX_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - GPIOx pin 0 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux0(&mut self) -> MUX_W<MUXL_SPEC> {
        MUX_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - GPIOx pin 1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux1(&mut self) -> MUX_W<MUXL_SPEC> {
        MUX_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - GPIOx pin 2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux2(&mut self) -> MUX_W<MUXL_SPEC> {
        MUX_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - GPIOx pin 3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux3(&mut self) -> MUX_W<MUXL_SPEC> {
        MUX_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - GPIOx pin 4 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux4(&mut self) -> MUX_W<MUXL_SPEC> {
        MUX_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - GPIOx pin 5 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux5(&mut self) -> MUX_W<MUXL_SPEC> {
        MUX_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - GPIOx pin 6 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux6(&mut self) -> MUX_W<MUXL_SPEC> {
        MUX_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - GPIOx pin 7 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mux7(&mut self) -> MUX_W<MUXL_SPEC> {
        MUX_W::new(self, 28)
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
#[doc = "GPIO muxing function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXL_SPEC;
impl crate::RegisterSpec for MUXL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxl::R`](R) reader structure"]
impl crate::Readable for MUXL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxl::W`](W) writer structure"]
impl crate::Writable for MUXL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXL to value 0"]
impl crate::Resettable for MUXL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
